#![warn(clippy::all, clippy::pedantic)]
use bracket_lib::prelude::*;
use rand::prelude::*;
use std::fmt;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 80;
const FRAME_DURATION: f32 = 150.0;
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

struct InsertionSort {
    vector: Vec<u16>,
    i: usize,
    j: usize,
    smallest: usize,
}

impl InsertionSort {
    fn new() -> Self {
        InsertionSort {
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
        if self.i == 0 {
            self.i = 1;
        }

        let key = self.vector[self.i];
        let mut j = self.i - 1;

        while self.vector[j] > key {
            self.vector.swap(j, j + 1);
            if j == 0 {
                break;
            }
            j -= 1;
        }
        self.i += 1;
    }
}

struct GnomeSort {
    vector: Vec<u16>,
    i: usize,
    j: usize,
    smallest: usize,
}

impl GnomeSort {
    fn new() -> Self {
        GnomeSort {
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
        if self.i == 0 {
            self.i += 1;
        }
        if self.vector[self.i] >= self.vector[self.i - 1] {
            self.i += 1;
        } else {
            self.vector.swap(self.i, self.i - 1);
            self.i -= 1;
        }
    }
}

enum Algorithm {
    Selection(SelectionSort),
    Bubble(BubbleSort),
    Insertion(InsertionSort),
    Gnome(GnomeSort),
}

impl Algorithm {
    fn render(&self, ctx: &mut BTerm) {
        match self {
            Algorithm::Selection(s) => s.render(ctx),
            Algorithm::Bubble(s) => s.render(ctx),
            Algorithm::Insertion(s) => s.render(ctx),
            Algorithm::Gnome(s) => s.render(ctx),
        }
    }

    fn sort(&mut self, ctx: &mut BTerm) {
        match self {
            Algorithm::Selection(s) => {
                for _i in 0..10 {
                    s.sort();
                    s.render(ctx);
                }
            }
            Algorithm::Bubble(s) => s.sort(),
            Algorithm::Insertion(s) => s.sort(),
            Algorithm::Gnome(s) => s.sort(),
        }
    }
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Algorithm::Selection(_) => write!(f, "selection sort"),
            Algorithm::Bubble(_) => write!(f, "bubble sort"),
            Algorithm::Insertion(_) => write!(f, "insertion sort"),
            Algorithm::Gnome(_) => write!(f, "gnome sort"),
        }
    }
}

// stores current state
struct State {
    frame_time: f32,
    algorithm: Algorithm,
}

impl State {
    fn new() -> Self {
        State {
            frame_time: 0.0,
            algorithm: Algorithm::Selection(SelectionSort::new()),
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(BLACK);

        // limits framrate
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
        }

        match ctx.key {
            Some(VirtualKeyCode::Key1) => println!("pressed 1"),
            Some(VirtualKeyCode::Space) => self.algorithm.sort(ctx),
            Some(VirtualKeyCode::R) => self.restart(),
            Some(_) => println!("Unbinded"),
            None => (),
        }

        ctx.print(0, SCREEN_HEIGHT - 2, &self.algorithm);
        self.algorithm.render(ctx);
    }

    fn restart(&mut self) {
        self.frame_time = 0.0;
        //self.data = Data::new();
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
