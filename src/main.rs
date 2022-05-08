#![warn(clippy::all, clippy::pedantic)]
use bracket_lib::prelude::*;
use rand::prelude::*;
use std::fmt;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 80;
const VECTOR_SIZE: u16 = 80;

fn create_random_vector() -> Vec<u16> {
    let mut vec: Vec<u16> = (1..=VECTOR_SIZE).collect();
    vec.shuffle(&mut thread_rng());
    vec
}

struct SelectionSort {
    vector: Vec<u16>,
    i: usize,
    j: usize,
    smallest: usize,
}

impl SelectionSort {
    fn new() -> Self {
        SelectionSort {
            vector: create_random_vector(),
            i: 0,
            j: 0,
            smallest: 0,
        }
    }

    fn render(&self, ctx: &mut BTerm) {
        for (i, n) in self.vector.iter().enumerate() {
            for y in 0..*n {
                if i == self.smallest {
                    ctx.set(i, y, RED, BLACK, to_cp437('#'));
                } else if i == self.j {
                    ctx.set(i, y, GREEN, BLACK, to_cp437('#'));
                } else if i < self.i {
                    ctx.set(i, y, YELLOW, BLACK, to_cp437('#'));
                } else {
                    ctx.set(i, y, LIGHT_BLUE, BLACK, to_cp437('#'));
                }
            }
        }
    }

    fn sort(&mut self) {
        if self.j < VECTOR_SIZE as usize {
            if self.vector[self.j] < self.vector[self.smallest] {
                self.smallest = self.j;
            }
            self.j += 1;
        } else if self.i < VECTOR_SIZE as usize {
            self.vector.swap(self.i, self.smallest);
            self.i += 1;
            self.j = self.i;
            self.smallest = self.i;
        }
    }
}

struct BubbleSort {
    vector: Vec<u16>,
    i: usize,
    j: usize,
    smallest: usize,
}

impl BubbleSort {
    fn new() -> Self {
        BubbleSort {
            vector: create_random_vector(),
            i: 0,
            j: 0,
            smallest: 0,
        }
    }

    fn render(&self, ctx: &mut BTerm) {
        for (i, n) in self.vector.iter().enumerate() {
            for y in 0..*n {
                if i == self.smallest {
                    ctx.set(i, y, RED, BLACK, to_cp437('#'));
                } else if i == self.i {
                    ctx.set(i, y, GREEN, BLACK, to_cp437('#'));
                } else {
                    ctx.set(i, y, YELLOW, BLACK, to_cp437('#'));
                }
            }
        }
    }
    fn sort(&mut self) {
        let range = self.vector.len();
        for j in 0..range - self.i - 1 {
            if self.vector[j] > self.vector[j + 1] {
                self.vector.swap(j, j + 1);
            }
        }
        self.i += 1;
    }
}

// stores current state
struct State {
    algorithm: SelectionSort,
}

impl State {
    fn new() -> Self {
        State {
            algorithm: SelectionSort::new(),
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        match ctx.key {
            Some(VirtualKeyCode::Key1) => println!("pressed 1"),
            Some(VirtualKeyCode::Key2) => self.algorithm = BubbleSort::new(),

            Some(VirtualKeyCode::Space) => {
                self.algorithm.sort();
                self.algorithm.render(ctx);
            }
            Some(VirtualKeyCode::R) => self.restart(ctx),

            Some(_) => println!("Unbinded"),
            None => (),
        }
    }

    fn restart(&mut self, ctx: &mut BTerm) {
        self.algorithm.render(ctx);
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
