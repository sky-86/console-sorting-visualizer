#![warn(clippy::all, clippy::pedantic)]
use bracket_lib::prelude::*;
use rand::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 80;
const FRAME_DURATION: f32 = 75.0;
const VECTOR_SIZE: u16 = 80;

// types of game states
enum GameMode {
    Menu,
    Playing,
    End,
}

struct Numbers {
    vector: Vec<u16>,
    current: usize,
    smallest: usize,
}

impl Numbers {
    fn new() -> Self {
        let mut vec: Vec<u16> = (1..=VECTOR_SIZE).collect();
        vec.shuffle(&mut thread_rng());

        Numbers {
            vector: vec,
            current: 0,
            smallest: 0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        for (i, n) in self.vector.iter().enumerate() {
            for y in 0..*n {
                if i == self.smallest {
                    ctx.set(i, y, RED, BLACK, to_cp437('#'));
                } else if i == self.current {
                    ctx.set(i, y, GREEN, BLACK, to_cp437('#'));
                } else {
                    ctx.set(i, y, YELLOW, BLACK, to_cp437('#'));
                }
            }
        }
    }

    fn step(&mut self) {
        let mut min = self.current;
        for j in self.current..self.vector.len() {
            if self.vector[j] < self.vector[min] {
                min = j;
            }
        }

        self.vector.swap(self.current, min);
        self.smallest = min;
        self.current += 1;
    }
}

// stores current state
struct State {
    frame_time: f32,
    mode: GameMode,
    numbers: Numbers,
}

impl State {
    fn new() -> Self {
        State {
            frame_time: 0.0,
            mode: GameMode::Menu,
            numbers: Numbers::new(),
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);

        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            if self.numbers.current < VECTOR_SIZE as usize {
                self.numbers.step();
            }
        }

        if let Some(VirtualKeyCode::R) = ctx.key {
            self.restart();
        }

        self.numbers.render(ctx);
    }

    fn restart(&mut self) {
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.numbers = Numbers::new();
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Sorting Visualizer");
        ctx.print_centered(8, "(S) Start");
        ctx.print_centered(9, "(Q) Quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::S => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                // do nothing if they press anything else
                _ => {}
            }
        }
    }

    fn done(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                // do nothing if they press anything else
                _ => {}
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.done(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)
        .unwrap()
        .with_title("Sorting Visualizer")
        .build()?;

    main_loop(context, State::new())
}
