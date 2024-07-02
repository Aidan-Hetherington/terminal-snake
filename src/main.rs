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

enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct GameState {
    pub dimensions: Vec2,
    pub snake: Vec<Vec2>,
    pub score: u32,
    pub food: Vec2
}

impl GameState {
    pub fn new(dimensions: Vec2) -> GameState {
        // Get y height of starting snake
        let y_pos = (dimensions.y / 2) + 1;
        // Populate snake array
        let snake = vec![Vec2::xy(0, y_pos), Vec2::xy(1, y_pos), Vec2::xy(2, y_pos)];
        
        // Place food
        let mut food = Vec2::xy(rand::thread_rng().gen_range(0..dimensions.x), rand::thread_rng().gen_range(0..dimensions.y));
        // If food is in the snake
        while snake.contains(&mut food) {
            // Try a new position
            food = Vec2::xy(rand::thread_rng().gen_range(0..dimensions.x), rand::thread_rng().gen_range(0..dimensions.y));
        }

        return GameState {
            dimensions: dimensions,
            snake: snake,
            score: 0,
            food: food,
        }
    }

    fn place_food(&self) -> Vec2 {
        // Set food position
        let mut food = Vec2::xy(rand::thread_rng().gen_range(0..self.dimensions.x), rand::thread_rng().gen_range(0..self.dimensions.y));
        // If food is in the snake
        while self.snake.contains(&mut food) {
            // Try a new position
            food = Vec2::xy(rand::thread_rng().gen_range(0..self.dimensions.x), rand::thread_rng().gen_range(0..self.dimensions.y));
        }

        return food;
    }

    pub fn update(&mut self, dir: Direction) {
        let new_head = match dir {
            Direction::Up => {
                let mut pos = *self.snake.first().expect("[Err] The GameState.snake array should never be empty by the time the update function is called");
                pos.y += 1;
                pos
            }
            Direction::Down => {
                let mut pos = *self.snake.first().expect("[Err] The GameState.snake array should never be empty by the time the update function is called");
                pos.y -= 1;
                pos
            },
            Direction::Right => {
                let mut pos = *self.snake.first().expect("[Err] The GameState.snake array should never be empty by the time the update function is called");
                pos.x += 1;
                pos
            },
            Direction::Left => {
                let mut pos = *self.snake.first().expect("[Err] The GameState.snake array should never be empty by the time the update function is called");
                pos.x -= 1;
                pos
            }
        };
        self.snake.insert(0, new_head);

        if self.food == *self.snake.first().expect(
            "[Err] The GameState.snake array should never be 
            empty by the time the update function is called") {
                self.place_food();
        } else {
            self.snake.pop();
        }
    }
}

fn main() -> io::Result<()> {
    crossterm::terminal::enable_raw_mode().expect("[Err] Failed to enable raw mode, quitting"); // Enable raw mode

    let mut app = App::default();
    let win_size = app.window().size();
    let state = GameState::new(win_size);
    app.run(|app_state: &mut State, window: &mut Window| {
        // `poll()` waits for an `Event` for a given time period
        if poll(Duration::from_millis(500)).unwrap() {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read().unwrap() {
                Event::Key(event) => {
                    // If q or esccape pressed then quit
                    if event.code == KeyCode::Char('q') {
                    }
                },
                _ => {
                    // Do nothing, no keys were pressed
                }
            }
        }
    });

    crossterm::terminal::disable_raw_mode().expect("[Err] Failed to disable raw mode, expect wierd terminal behaviour"); // Disable raw mode

    // Closed successfully
    Ok(())
}