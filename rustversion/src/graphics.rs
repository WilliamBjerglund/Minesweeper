use std::sync::Arc;

use winit::window::Window;

pub struct Graphics {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    // We now need to add our shaders and rules for geometry
    render_pipeline: wgpu::RenderPipeline,
}

impl Graphics {
    pub fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .unwrap();

        let (device, queue) =
            pollster::block_on(adapter.request_device(&Default::default())).unwrap();

        let size = window.inner_size();

        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();

        surface.configure(&device, &config);

        // We want to load our shader.wgsl and create our module
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        // Here we describe everything the shaders can access it is empty since my vertex positions and colors are currently hardcoded inside the shader.
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        // now i need to connect the shader function to the rendering pipeline.
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render pipeline"),
            layout: Some(&pipeline_layout),

            // now we need to configure the vertex sahder.
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },

            // Now for the fragment shader
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),

                targets: &[Some(wgpu::ColorTargetState {
                    // Must match the window surface format.
                    format: config.format,

                    blend: Some(wgpu::BlendState::REPLACE),

                    write_mask: wgpu::ColorWrites::ALL,
                })],

                compilation_options: Default::default(),
            }),

            // Every group of three vertices becomes a triangle.
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,

                ..Default::default()
            },

            // No depth buffer needed for this simple 2D shape.
            depth_stencil: None,

            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        });

        Self {
            surface,
            device,
            queue,
            config,
            render_pipeline,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.config.width = width;
        self.config.height = height;

        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(&mut self) {
        let frame = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame)
            | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,

            _ => return,
        };

        let view = frame.texture.create_view(&Default::default());

        let mut encoder = self.device.create_command_encoder(&Default::default());

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });

            // Now we select the pipeline we created containing vs and fs main
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..6, 0..1);
        }

        self.queue.submit([encoder.finish()]);
        frame.present();
    }
}
