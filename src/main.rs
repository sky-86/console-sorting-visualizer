#![warn(clippy::all, clippy::pedantic)]
use bracket_lib::prelude::*;
use std::collections::HashMap;
mod algo;
use crate::algo::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 80;
const FRAME_DURATION: f32 = 75.0;
const MAX_ITERATIONS: usize = 100;

// stores current state
struct State {
    frame_time: f32,
    current_key: String,
    algorithms: HashMap<String, Box<dyn Algorithm>>,
    loop_iterations: usize,
}

impl State {
    fn new() -> Self {
        // creating two vectors and combining them seems to work better
        let keys = vec!["selection".into(), "bubble".into()];
        let algos: Vec<Box<dyn Algorithm>> =
            vec![Box::new(SelectionSort::new()), Box::new(BubbleSort::new())];

        State {
            frame_time: 0.0,
            current_key: "selection".into(),
            algorithms: keys.into_iter().zip(algos.into_iter()).collect(),
            loop_iterations: 10,
        }
    }

    // gets called on every game tick
    fn play(&mut self, ctx: &mut BTerm) {
        // checks for user key presses
        match ctx.key {
            Some(VirtualKeyCode::Key1) => self.current_key = "selection".into(),
            Some(VirtualKeyCode::Key2) => self.current_key = "bubble".into(),

            Some(VirtualKeyCode::Space) => {
                // add the ability to change the loop range
                //
                for _i in 0..self.loop_iterations {
                    self.algorithms.get_mut(&self.current_key).unwrap().sort();
                }
            }
            Some(VirtualKeyCode::R) => self.restart(),

            // press up and down to change loop iterations
            Some(VirtualKeyCode::Up) => {
                if self.loop_iterations < MAX_ITERATIONS {
                    self.loop_iterations += 1;
                }
            }
            Some(VirtualKeyCode::Down) => {
                if self.loop_iterations != 1 {
                    self.loop_iterations -= 1;
                }
            }

            Some(_) => println!("Unbinded"),
            None => (),
        }

        // limits the amount of times the screen is being rendered
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;

            ctx.cls_bg(BLACK);
            self.algorithms[&self.current_key].render(ctx);
            ctx.print(2, SCREEN_HEIGHT - 2, &self.current_key);
            ctx.print(SCREEN_WIDTH - 4, SCREEN_HEIGHT - 2, self.loop_iterations);
        }
    }

    fn restart(&mut self) {
        self.algorithms.get_mut(&self.current_key).unwrap().reset();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.play(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)
        .unwrap()
        .with_title("Sorting Visualizer")
        .build()?;

    main_loop(context, State::new())
}
