use std::vec;
// Non-blocking keyboard input
use std::{time::Duration, io};
// Read keyboard events
use crossterm::event::{poll, read, Event, KeyCode};
// Drawing to terminal
use ruscii::app::{App, State};
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Style, Window};
// Randomly place an apple
use rand::{self, prelude::*};

struct GameState {
    pub dimensions: Vec2,
    pub snake: Vec<Vec2>,
    pub head: Vec2,
    pub score: u32,
    pub food: Vec2
}

impl GameState {
    pub fn new(dimensions: Vec2) -> GameState {
        // Get y height of starting snake
        let y_pos = (dimensions.y / 2) + 1;
        // Populate snake array
        let snake = vec![Vec2::xy(0, y_pos), Vec2::xy(1, y_pos), Vec2::xy(2, y_pos)];
        // Set head pos
        let head = Vec2::xy(2, y_pos);
        // Set food position
        let mut food = Vec2::xy(rand::thread_rng().gen_range(0..dimensions.x), rand::thread_rng().gen_range(0..dimensions.y));
        // If food is in the snake
        while snake.contains(&mut food) {
            // Try a new position
            food = Vec2::xy(rand::thread_rng().gen_range(0..dimensions.x), rand::thread_rng().gen_range(0..dimensions.y));
        }

        return GameState {
            dimensions: dimensions,
            snake: snake,
            head: head,
            score: 0,
            food: food,
        }
    }
}

fn main() -> io::Result<()> {
    match crossterm::terminal::enable_raw_mode() {
        Ok(()) => (), // Succeeded in enabling raw mode
        Err(e) => { // Failed to enable raw mode
            eprintln!("[Err] Failed to enable raw mode, Quitting")
        }
    }

    let app = App::default();
    let win_size = app.window().size();
    let state = GameState::new(win_size);

    // `poll()` waits for an `Event` for a given time period
    if poll(Duration::from_millis(500))? {
        // It's guaranteed that the `read()` won't block when the `poll()`
        // function returns `true`
        match read()? {
            Event::Key(event) => {
                // If q or esccape pressed then quit
                if event.code == KeyCode::Char('q') {
                    break;
                }
            },
            _ => {
                // Do nothing, no keys were pressed
            }
        }
    }

    match crossterm::terminal::disable_raw_mode() {
        Ok(()) => (),
        Err(e) => {
            eprintln!("[Err] Failed to disable raw mode: you may need to restart your terminal")
        }
    }

    // Closed successfully
    Ok(())
}