use std::{mem, time::Duration};

use egui::{pos2, vec2, Color32, Pos2, Rect, Rounding, Shape, Vec2};
use instant::Instant;
use ndarray::Array2;

use rand::{prelude::ThreadRng, thread_rng, Rng};

const NEIGHBOURHOOD: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, -1),
    (0, 1),
];

pub struct World {
    pub data: Array2<u8>,
    pub tmp: Array2<u8>,
    pub block_size: f32,
    pub pos_shift: Vec2,
    pub shift: Vec2,
    pub fps: i32,
    pub threshold: f32,
    pub use_shift: bool,
    speed: u128,
    num_of_blocks: usize,
    last_frame_time: Instant,
    rng: ThreadRng,
}

impl World {
    pub fn new(num_of_blocks: usize) -> Self {
        let mut _data = Array2::<u8>::zeros((num_of_blocks, num_of_blocks));
        let mut _tmp = Array2::<u8>::zeros((num_of_blocks, num_of_blocks));

        Self {
            data: _data,
            tmp: _tmp,
            block_size: 2.0,
            pos_shift: vec2(0.0, 0.0),
            shift: vec2(0.0, 0.0),
            fps: 60,
            threshold: 0.5,
            speed: World::fps_to_speed(60.0),
            num_of_blocks: num_of_blocks,
            last_frame_time: Instant::now(),
            rng: thread_rng(),
            use_shift: false,
        }
    }

    pub fn rand_generate(&mut self) {
        for cell in &mut self.data {
            *cell = if self.rng.gen_bool(self.threshold.into()) {
                1u8
            } else {
                0u8
            };
        }
    }

    pub fn gen_shapes(&self, shapes: &mut Vec<Shape>, rect: Rect) {
        for y in 0..self.data.dim().0 {
            for x in 0..self.data.dim().1 {
                shapes.push(Shape::rect_filled(
                    Rect {
                        min: rect.min
                            + vec2(self.block_size * x as f32, self.block_size * y as f32)
                            + self.pos_shift,
                        max: rect.min
                            + vec2(
                                self.block_size * (x + 1) as f32,
                                self.block_size * (y + 1) as f32,
                            )
                            + self.pos_shift,
                    },
                    Rounding::none(),
                    if self.data[[y, x]] == 1u8 {
                        Color32::BLACK
                    } else {
                        Color32::WHITE
                    },
                ));
            }
        }
    }

    pub fn fps_to_speed(fps: f32) -> u128 {
        Duration::new(0, (1000000000.0 / fps) as u32).as_millis()
    }

    pub fn bounds_valid(&self, block: Vec2) -> bool {
        if (block.x as i32) >= 0
            && (block.x as usize) < self.num_of_blocks
            && (block.y as i32) >= 0
            && (block.y as usize) < self.num_of_blocks
        {
            return true;
        }
        return false;
    }

    pub fn update(&mut self) {
        let duration_since_last_frame = Instant::now().duration_since(self.last_frame_time);
        if duration_since_last_frame.as_millis().lt(&self.speed) {
            return;
        }

        self.last_frame_time = Instant::now();

        let mut sum;
        let mut curr;
        for y in 0..self.tmp.dim().0 as i32 {
            for x in 0..self.tmp.dim().1 as i32 {
                sum = 0;
                for step in NEIGHBOURHOOD {
                    if y + step.0 < 0
                        || x + step.1 < 0
                        || y + step.0 == self.tmp.dim().0 as i32
                        || x + step.1 == self.tmp.dim().1 as i32
                    {
                        continue;
                    }
                    if self.data[[(y + step.0) as usize, (x + step.1) as usize]] == 1u8 {
                        sum += 1;
                    }
                }

                curr = (self.data[[y as usize, x as usize]], sum);
                self.tmp[[y as usize, x as usize]] = match curr {
                    (1, 2 | 3) => 1u8,
                    (0, 3) => 1u8,
                    _ => 0u8,
                };
            }
        }
        mem::swap(&mut self.data, &mut self.tmp);
    }

    pub fn clear(&mut self) {
        for cell in &mut self.data {
            *cell = 0u8;
        }
    }

    pub fn update_speed(&mut self) {
        self.speed = World::fps_to_speed(self.fps as f32);
    }

    pub fn update_pos(&mut self) {
        self.pos_shift.x = -self.block_size * self.shift.x;
        self.pos_shift.y = -self.block_size * self.shift.y;
    }

    pub fn transform_cell(&mut self, pointer_pos: Option<Pos2>, clip_rect: Rect) {
        if let Some(pos) = pointer_pos {
            let block = self.get_block_pos(pos - pos2(clip_rect.left(), clip_rect.top()));

            if self.bounds_valid(block) {
                self.data[[block.y as usize, block.x as usize]] = 1u8;
            }
        }
    }

    fn get_block_pos(&self, pos: Vec2) -> Vec2 {
        return ((pos - self.pos_shift) / self.block_size).floor();
    }
}
