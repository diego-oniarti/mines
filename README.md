# Minesweeper in your terminal
A simple minesweeper written in Rust and playable from the terminal.  
The board is not guaranteed to be solvable without guessing.

## Controls
Move the cursor with either the arrow keys or the Vim way, with hjkl  
Holding down Ctrl or Shift while moving will jump to the end of the region the cursor is in  
Open cells with `f` or flag them with `d`  
Quit the game with `q`  
Refresh the screen with `r`  

## Options
Usage: mines [OPTIONS]  

Options:  
  -s, --size <SIZE>  Width and height of the board in the format WIDTHxHEIGHT (e.g., 50x30) [default: 50x30]  
  -p, --prob <PROB>  Probability of bombs in the grid (a value between 0.0 and 1.0) [default: 0.1]  
  -h, --help         Print help  
  -V, --version      Print version  

