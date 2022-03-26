mod world;

use eframe::{
    egui::{self, Button, Painter, Sense, Slider, SliderOrientation},
    epaint::{vec2, Pos2},
    epi::{self},
};

use egui::{Color32, Rounding, Shape};
use world::World;

pub struct App {
    running: bool,
    world: World,
}

impl Default for App {
    fn default() -> Self {
        let block_size = 2.0;
        let num_of_blocks = (800.0 / block_size) as usize;
        Self {
            running: false,
            world: World::new(num_of_blocks),
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Vaja 1"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        let Self { world, running } = self;

        ctx.request_repaint();

        egui::Window::new("Menu").resizable(false).show(ctx, |ui| {
            if ui
                .add(
                    Slider::new(&mut world.block_size, 1.0..=30.0)
                        .step_by(1.0)
                        .orientation(SliderOrientation::Horizontal)
                        .text("Block size"),
                )
                .changed()
            {
                world.update_pos();
            }

            ui.separator();

            ui.horizontal(|ui| {
                if ui.add(Button::new("Toggle")).clicked() {
                    *running = !*running;
                }

                if ui
                    .add_enabled(!*running, Button::new("Generate new"))
                    .clicked()
                {
                    world.rand_generate();
                }

                if ui.add_enabled(!*running, Button::new("Clear")).clicked() {
                    world.clear();
                }
            });

            if ui
                .add(
                    Slider::new(&mut world.fps, 1..=360)
                        .step_by(1.0)
                        .orientation(SliderOrientation::Horizontal)
                        .text("FPS"),
                )
                .changed()
            {
                world.update_speed();
            }

            if ui
                .add(
                    Slider::new(&mut world.shift.x, 0.0..=400.0)
                        .step_by(1.0)
                        .orientation(SliderOrientation::Horizontal)
                        .text("Shift x"),
                )
                .changed()
            {
                world.update_pos();
            }

            if ui
                .add(
                    Slider::new(&mut world.shift.y, 0.0..=400.0)
                        .step_by(1.0)
                        .orientation(SliderOrientation::Horizontal)
                        .text("Shift y"),
                )
                .changed()
            {
                world.update_pos();
            }

            ui.separator();

            ui.add(
                Slider::new(&mut world.threshold, 0.0..=1.0)
                    .step_by(0.01)
                    .orientation(SliderOrientation::Horizontal)
                    .text("Threshold"),
            );

            ui.separator();
        });

        egui::Window::new("Display")
            .default_size(vec2(600.0, 600.0))
            .default_pos(Pos2 { x: 600.0, y: 200.0 })
            .show(ctx, |ui| {
                let painter = Painter::new(
                    ui.ctx().clone(),
                    ui.layer_id(),
                    ui.available_rect_before_wrap(),
                );

                ui.expand_to_include_rect(painter.clip_rect());
                let rect = painter.clip_rect();
                let mut shapes = Vec::new();

                if ui
                    .interact(
                        rect,
                        ui.id(),
                        Sense {
                            click: true,
                            drag: true,
                            focusable: true,
                        },
                    )
                    .dragged()
                {
                    world.transform_cell(
                        ui.interact(
                            ui.clip_rect(),
                            ui.id(),
                            Sense {
                                click: true,
                                drag: false,
                                focusable: false,
                            },
                        )
                        .hover_pos(),
                        rect,
                    )
                };

                if *running {
                    world.update();
                }

                shapes.push(Shape::rect_filled(rect, Rounding::none(), Color32::WHITE));

                world.gen_shapes(&mut shapes, rect);

                painter.extend(shapes);
            });
    }
}
