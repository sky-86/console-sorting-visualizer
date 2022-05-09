use bracket_lib::prelude::*;
use rand::prelude::*;

const VECTOR_SIZE: u16 = 80;

fn create_random_vector() -> Vec<u16> {
    let mut vec: Vec<u16> = (1..=VECTOR_SIZE).collect();
    vec.shuffle(&mut thread_rng());
    vec
}

// The shared functionality of the different algorithms
pub trait Algorithm {
    fn render(&self, ctx: &mut BTerm);
    fn sort(&mut self);
    fn reset(&mut self);
}

pub struct SelectionSort {
    vector: Vec<u16>,
    i: usize,
    j: usize,
    smallest: usize,
}

impl SelectionSort {
    pub fn new() -> Self {
        SelectionSort {
            vector: create_random_vector(),
            i: 0,
            j: 0,
            smallest: 0,
        }
    }
}

// implement the shared functions
impl Algorithm for SelectionSort {
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

    // no loops, should sort one step at a time
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

    fn reset(&mut self) {
        self.vector = create_random_vector();
        self.i = 0;
        self.j = 0;
        self.smallest = 0;
    }
}

pub struct BubbleSort {
    vector: Vec<u16>,
    i: usize,
    j: usize,
}

impl BubbleSort {
    pub fn new() -> Self {
        BubbleSort {
            vector: create_random_vector(),
            i: 0,
            j: 0,
        }
    }
}

impl Algorithm for BubbleSort {
    fn render(&self, ctx: &mut BTerm) {
        for (i, n) in self.vector.iter().enumerate() {
            for y in 0..*n {
                if i == self.j || i == self.j + 1 {
                    ctx.set(i, y, GREEN, BLACK, to_cp437('#'));
                } else if i > VECTOR_SIZE as usize - self.i {
                    ctx.set(i, y, YELLOW, BLACK, to_cp437('#'));
                } else {
                    ctx.set(i, y, LIGHT_BLUE, BLACK, to_cp437('#'));
                }
            }
        }
    }

    fn sort(&mut self) {
        if self.i == 80 {
            return;
        }
        if self.j < VECTOR_SIZE as usize - self.i - 1 {
            if self.vector[self.j] > self.vector[self.j + 1] {
                self.vector.swap(self.j, self.j + 1);
            }
            self.j += 1;
        } else if self.i < VECTOR_SIZE as usize {
            self.i += 1;
            self.j = 0;
        }
    }

    fn reset(&mut self) {
        self.vector = create_random_vector();
        self.i = 0;
        self.j = 0;
    }
}

pub struct InsertionSort {
    vector: Vec<u16>,
    key: usize,
    i: usize,
    j: usize,
}

impl InsertionSort {
    pub fn new() -> Self {
        InsertionSort {
            vector: create_random_vector(),
            key: 0,
            i: 0,
            j: 0,
        }
    }
}

impl Algorithm for InsertionSort {
    fn render(&self, ctx: &mut BTerm) {
        for (i, n) in self.vector.iter().enumerate() {
            for y in 0..*n {
                if i == self.j || i == self.j + 1 {
                    ctx.set(i, y, GREEN, BLACK, to_cp437('#'));
                } else if i > VECTOR_SIZE as usize - self.i - 1 {
                    ctx.set(i, y, YELLOW, BLACK, to_cp437('#'));
                } else {
                    ctx.set(i, y, LIGHT_BLUE, BLACK, to_cp437('#'));
                }
            }
        }
    }

    fn sort(&mut self) {
        if self.vector[self.j] as usize > self.key {
            if self.vector[self.j] > self.vector[self.j + 1] {
                self.vector.swap(self.j, self.j + 1);
            }
            self.j += 1;
        } else if self.i < VECTOR_SIZE as usize {
            self.i += 1;
            self.j = 0;
        }
    }

    fn reset(&mut self) {
        self.vector = create_random_vector();
        self.i = 0;
        self.j = 0;
    }
}
