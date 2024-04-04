use std::f32::consts::PI;

use bevy::math::{Quat, Vec3};

use crate::{AntAction, Food, Pheromone, Player};

pub struct Natural;

impl Natural {
    pub fn ant_action(

        direction: Vec3,

        nearby_pheromones: &[(Pheromone, Player, Vec3)],
        nearby_food: &[(Food, Vec3)],
    ) -> AntAction {
        let nearest_food = nearby_food.iter().reduce(|nearest, next| {
            if nearest.1.length() > next.1.length() {
                next
            } else {
                nearest
            }
        });


        let p = rand::random::<f32>();

        let threshold = if nearby_pheromones.len() > 15 && nearby_pheromones.len() < 60 {
            0.95
        } else if nearby_food.len() > 0 && nearby_pheromones.len() < 80 {
            0.8
        } else {
            0.9999
        };

        if p < threshold {
            let da = (rand::random::<f32>() - 0.5) * PI / 20.;

            let rotation = Quat::from_rotation_z(da);

            let new_direction = if let Some(nearest_food) = nearest_food {
                let r = nearest_food.1.length();
                rotation * direction + 10. / (r * r) * nearest_food.1 * Vec3::new(1., 1., 0.)
            } else {
                nearby_pheromones
                    .iter()
                    .fold(rotation * direction, |dir, (_, _, pos)| {
                        let r = pos.length();

                        dir + 0.1 / (r * r) * *pos * Vec3::new(1., 1., 0.)
                    })
                    .normalize()
            };

            let (axis, angle) =
                Quat::from_rotation_arc(direction, new_direction.normalize()).to_axis_angle();

            let angle = axis.z * angle;
            let angle = if angle < -PI / 8. {
                -PI / 8.
            } else if angle > PI / 8. {
                PI / 8.
            } else {
                angle
            };

            AntAction::Rotate(Quat::from_rotation_z(angle))

        } else {
            let p = rand::random::<f32>();
            let pheromone = if p > 0.5 {
                Pheromone::Green
            } else {
                Pheromone::Red
            };
            AntAction::DropPheromone(pheromone)
        }
    }
}
