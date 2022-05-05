#![warn(clippy::all, clippy::pedantic)]
use bracket_lib::prelude::*;
use rand::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 80;
const FRAME_DURATION: f32 = 75.0;
const VECTOR_SIZE: u16 = 80;

struct Data {
    vector: Vec<u16>,
    current: usize,
    smallest: usize,
    i: usize,
    j: usize,
}

impl Data {
    fn new() -> Self {
        let mut vec: Vec<u16> = (1..=VECTOR_SIZE).collect();
        vec.shuffle(&mut thread_rng());

        Data {
            vector: vec,
            current: 0,
            smallest: 0,
            i: 0,
            j: 0,
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

    fn sort(&mut self, algo: &str) {
        match algo {
            "selection" => self.selection_sort(),
            "bubble" => self.bubble_sort(),
            "insert" => self.insertion_sort(),
            "gnome" => self.gnome_sort(),
            _ => println!("Error, sorting algo"),
        }
    }

    fn selection_sort(&mut self) {
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

    fn bubble_sort(&mut self) {
        let range = self.vector.len();
        for j in 0..range - self.current - 1 {
            if self.vector[j] > self.vector[j + 1] {
                self.vector.swap(j, j + 1);
            }
        }
        self.current += 1;
    }

    fn insertion_sort(&mut self) {
        if self.current == 0 {
            self.current = 1;
        }

        let key = self.vector[self.current];
        let mut j = self.current - 1;

        while self.vector[j] > key {
            self.vector.swap(j, j + 1);
            if j == 0 {
                break;
            }
            j -= 1;
        }
        self.current += 1;
    }

    fn gnome_sort(&mut self) {
        if self.current == 0 {
            self.current += 1;
        }
        if self.vector[self.current] >= self.vector[self.current - 1] {
            self.current += 1;
        } else {
            self.vector.swap(self.current, self.current - 1);
            self.current -= 1;
        }
    }
}

// stores current state
struct State {
    frame_time: f32,
    data: Data,
    algorithm: String,
}

impl State {
    fn new() -> Self {
        State {
            frame_time: 0.0,
            data: Data::new(),
            algorithm: String::from("selection"),
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);

        // limits framrate
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
        }

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.algorithm = String::from("selection");
        } else if let Some(VirtualKeyCode::Key2) = ctx.key {
            self.algorithm = String::from("bubble");
        } else if let Some(VirtualKeyCode::Key3) = ctx.key {
            self.algorithm = String::from("insert");
        } else if let Some(VirtualKeyCode::Key4) = ctx.key {
            self.algorithm = String::from("gnome");
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            if self.data.current < VECTOR_SIZE as usize {
                self.data.sort(&self.algorithm);
            }
        }

        if let Some(VirtualKeyCode::R) = ctx.key {
            self.restart();
        }

        ctx.print(0, SCREEN_HEIGHT - 2, &self.algorithm);
        self.data.render(ctx);
    }

    fn restart(&mut self) {
        self.frame_time = 0.0;
        self.data = Data::new();
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
