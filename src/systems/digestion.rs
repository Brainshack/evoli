use amethyst::renderer::{debug_drawing::DebugLines, palette::Srgba};
use amethyst::{core::Time, core::Transform, ecs::*};
use std::f32;

use crate::components::digestion::{Digestion, Fullness};
use crate::components::combat::Health;

pub struct DigestionSystem;

impl<'s> System<'s> for DigestionSystem {
    type SystemData = (
        ReadStorage<'s, Digestion>,
        WriteStorage<'s, Fullness>,
        Read<'s, Time>,
    );

    fn run(&mut self, (digestions, mut fullnesses, time): Self::SystemData) {
        let delta_time = time.delta_seconds();
        for (digestion, fullness) in (&digestions, &mut fullnesses).join() {
            let burned = digestion.nutrition_burn_rate * delta_time;
            let new_value = fullness.value - burned;
            fullness.value = new_value;
        }
    }
}

pub struct StarvationSystem;

// Entities die if their fullness reaches zero (or less).
impl<'s> System<'s> for StarvationSystem {
    type SystemData = (
        ReadStorage<'s, Fullness>,
        WriteStorage<'s, Health>,
    );

    fn run(&mut self, (fullnesses, mut healths): Self::SystemData) {
        for (fullness, mut health) in (&fullnesses, &mut healths).join() {
            if fullness.value < f32::EPSILON {
                health.value = 0.0;
            }
        }
    }
}

pub struct DebugFullnessSystem;

impl<'s> System<'s> for DebugFullnessSystem {
    type SystemData = (
        ReadStorage<'s, Fullness>,
        ReadStorage<'s, Transform>,
        Write<'s, DebugLines>,
    );

    fn run(&mut self, (fullnesses, locals, mut debug_lines): Self::SystemData) {
        for (fullness, local) in (&fullnesses, &locals).join() {
            let pos = local.global_matrix();
            debug_lines.draw_line(
                [pos[(3, 0)], pos[(3, 1)], 0.0].into(),
                [pos[(3, 0)] + fullness.value / 100.0, pos[(3, 1)], 0.0].into(),
                Srgba::new(0.0, 1.0, 0.0, 1.0),
            )
        }
    }
}
