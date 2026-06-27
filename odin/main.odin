package main

import "core:fmt"
import "core:math/rand"
import "core:os"
import "core:strconv"
import "core:strings"
import rl "vendor:raylib"

CELL_SIZE :: 40

Game :: struct {
	width:    int,
	height:   int,
	cells:    []int,
	revealed: []bool,
	over:     bool,
}

get_idx :: proc(self: ^Game, x, y: int) -> int {
	if x < 0 || x >= self.width || y < 0 || y >= self.height do return -1
	return y * self.width + x
}

game_init :: proc(self: ^Game, w, h, mines: int) {
	self.width = w
	self.height = h
	self.cells = make([]int, w * h)
	self.revealed = make([]bool, w * h)

	planted := 0
	for planted < mines {
		idx := rand.int_max(w * h)
		if self.cells[idx] == -1 do continue
		self.cells[idx] = -1
		planted += 1
	}

	for i in 0 ..< len(self.cells) {
		if self.cells[i] == -1 do continue
		x, y := i % w, i / w
		count := 0

		for dy in -1 ..= 1 {
			for dx in -1 ..= 1 {
				idx := get_idx(self, x + dx, y + dy)
				if idx != -1 && self.cells[idx] == -1 do count += 1
			}
		}
		self.cells[i] = count
	}
}

game_free :: proc(self: ^Game) {
	delete(self.cells)
	delete(self.revealed)
}

main :: proc() {
	buf: [64]byte
	fmt.print("Enter layout (width height mines): ")
	n, _ := os.read(os.stdin, buf[:])
	config := strings.fields(string(buf[:n]))
	if len(config) < 3 {
		fmt.println("Invalid parameters.")
		return
	}

	w, _ := strconv.parse_int(config[0])
	h, _ := strconv.parse_int(config[1])
	mines, _ := strconv.parse_int(config[2])

	game: Game
	game_init(&game, w, h, mines)
	defer game_free(&game)

	rl.InitWindow(i32(w * CELL_SIZE), i32(h * CELL_SIZE), "Minesweeper")
	rl.SetTargetFPS(60)

	for !rl.WindowShouldClose() {
		if !game.over && rl.IsMouseButtonPressed(.LEFT) {
			m_pos := rl.GetMousePosition()
			x := int(m_pos.x) / CELL_SIZE
			y := int(m_pos.y) / CELL_SIZE

			idx := get_idx(&game, x, y)
			if idx != -1 {
				game.revealed[idx] = true
				if game.cells[idx] == -1 do game.over = true
			}
		}


		rl.BeginDrawing()
		rl.ClearBackground(rl.GRAY)

		for i in 0 ..< len(game.cells) {
			x := i32(i % game.width) * CELL_SIZE
			y := i32(i / game.width) * CELL_SIZE


			if !game.revealed[i] {
				rl.DrawRectangle(x, y, CELL_SIZE - 2, CELL_SIZE - 2, rl.LIGHTGRAY)
			} else if game.cells[i] == -1 {
				rl.DrawRectangle(x, y, CELL_SIZE - 2, CELL_SIZE - 2, rl.RED)
				rl.DrawText("*", x + 15, y + 10, 20, rl.WHITE)
			} else {
				rl.DrawRectangle(x, y, CELL_SIZE - 2, CELL_SIZE - 2, rl.RAYWHITE)
				if game.cells[i] > 0 {
					num_str := fmt.ctprintf("%d", game.cells[i])
					rl.DrawText(num_str, x + 15, y + 10, 20, rl.DARKGRAY)
				}
			}
		}

		if game.over {
			rl.DrawRectangle(0, 0, i32(w * CELL_SIZE), i32(h * CELL_SIZE), rl.Fade(rl.BLACK, 0.5))
			rl.DrawText(
				"GAME OVER",
				i32((w * CELL_SIZE) / 2) - 70,
				i32((h * CELL_SIZE) / 2) - 10,
				24,
				rl.RED,
			)
		}

		rl.EndDrawing()
	}

	rl.CloseWindow()
}
