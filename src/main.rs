mod game;

use clap::Parser;
use crossterm::{
    cursor, event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers}, style::{Color, SetBackgroundColor, SetForegroundColor}, terminal::{self}, ExecutableCommand
};
use game::Game;
use std::{io::{stdout, Write}, time::Duration, thread};

/// Command line arguments for configuring the Minesweeper game.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Width and height of the board in the format WIDTHxHEIGHT (e.g., 50x30)
    #[arg(short='s', long, default_value = "50x30")] // Default size is 50x30
    size: String,

    /// Probability of bombs in the grid (a value between 0.0 and 1.0)
    #[arg(short='p', long, default_value_t = 0.1, value_parser = clap::value_parser!(f64))]
    prob: f64,
}

fn parse_size(size_str: &str) -> Option<(usize, usize)> {
    let parts: Vec<&str> = size_str.split('x').collect();
    if parts.len() == 2 {
        if let (Ok(width), Ok(height)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
            return Some((width, height));
        }
    }
    None
}

fn main() {
    // Parse command line arguments
    let args = Args::parse();

    // Parse the board size from the input argument or use the default
    let (map_width, map_height) = parse_size(&args.size).unwrap_or((50, 30)); // Default size is 50x30

    let bomb_probability = args.prob as f32; // Use the probability of bombs

    // Now pass map_width, map_height, and bomb_probability into your game
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(cursor::Hide).unwrap();

    let mut game: Game = Game::new(map_width, map_height, bomb_probability);

    // Clear the screen
    stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();

    game.draw();
    loop {
        // Handle input
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(KeyEvent { code, modifiers, kind, .. }) = event::read().unwrap() {
                if kind == KeyEventKind::Press {  // Only handle key presses
                    match (code, modifiers) {
                        (KeyCode::Up, KeyModifiers::NONE) | (KeyCode::Char('k'), KeyModifiers::NONE) => {
                            game.move_up();
                        }
                        (KeyCode::Down, KeyModifiers::NONE) | (KeyCode::Char('j'), KeyModifiers::NONE) => {
                            game.move_down();
                        }
                        (KeyCode::Left, KeyModifiers::NONE) | (KeyCode::Char('h'), KeyModifiers::NONE) => {
                            game.move_left();
                        }
                        (KeyCode::Right, KeyModifiers::NONE) | (KeyCode::Char('l'), KeyModifiers::NONE) => {
                            game.move_right();
                        }

                        (KeyCode::Char('k'), KeyModifiers::CONTROL) | (KeyCode::Up, KeyModifiers::CONTROL) | (KeyCode::Char('K'), KeyModifiers::SHIFT) => {
                            game.long_up();
                        }
                        (KeyCode::Char('j'), KeyModifiers::CONTROL) | (KeyCode::Down, KeyModifiers::CONTROL) | (KeyCode::Char('J'), KeyModifiers::SHIFT) => {
                            game.long_down();
                        }
                        (KeyCode::Char('h'), KeyModifiers::CONTROL) | (KeyCode::Left, KeyModifiers::CONTROL) | (KeyCode::Char('H'), KeyModifiers::SHIFT) => {
                            game.long_left();
                        }
                        (KeyCode::Char('l'), KeyModifiers::CONTROL) | (KeyCode::Right, KeyModifiers::CONTROL) | (KeyCode::Char('L'), KeyModifiers::SHIFT) => {
                            game.long_right();
                        }

                        (KeyCode::Char('x'), _) | (KeyCode::Char('f'), _) => {
                            game.click();
                        }
                        (KeyCode::Char('z'), _) | (KeyCode::Char('d'), _) => {
                            game.flag();
                        }
                        (KeyCode::Char('c'), _) | (KeyCode::Char('r'), _) => {
                            game.refresh();
                        }
                        // Quit with 'q' or Esc
                        (KeyCode::Char('q'), _) | (KeyCode::Esc, _) => break,
                        _ => {}
                    }
                }
            }
        }
        if game.is_lost() || game.is_won() {
            break;
        }

        // Sleep to prevent high CPU usage
        thread::sleep(Duration::from_millis(16)); // Roughly 60 FPS
    }
    game.draw();
    if game.is_won() {
        stdout.execute(cursor::MoveTo(0, map_height as u16)).unwrap();
        stdout.execute(SetBackgroundColor(Color::Green)).unwrap();
        stdout.execute(SetForegroundColor(Color::Black)).unwrap();
        write!(stdout, "You Won!").unwrap();
        stdout.flush().unwrap();
    }

    // Cleanup: Show the cursor again and disable raw mode
    stdout.execute(cursor::Show).unwrap();
    stdout.execute(SetForegroundColor(Color::Reset)).unwrap();
    stdout.execute(SetBackgroundColor(Color::Reset)).unwrap();
    terminal::disable_raw_mode().unwrap();
}
