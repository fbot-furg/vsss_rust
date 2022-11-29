use crate::{Vector, Goal, Obstacle};

pub struct PotentialField {
    goal: Goal,
    obstacles: Vec<Obstacle>
}

impl PotentialField {
    pub fn new(goal: Goal, obstacles: Vec<Obstacle>) -> Self {
        Self {
            goal: goal,
            obstacles: obstacles
        }
    }

    pub fn calculate_force(&self, point: Vector) -> Vector {
        //Attractive potential
        let mut force = Vector {
            x: self.goal.point.x - point.x,
            y: self.goal.point.y - point.y,
        };

        //Repulsive potential
        for obstacle in &self.obstacles {
            let distance = point.distance_to(&obstacle.point);

            let mut repulsive_force = Vector {
                x: obstacle.point.x - point.x,
                y: obstacle.point.y - point.y,
            };

            //aply repulsive potential only if the robot is inside the obstacle
            if distance < obstacle.radius {
                //Distance power 2
                repulsive_force.x *= 0.15 / distance.powi(2);
                repulsive_force.y *= 0.15 / distance.powi(2);

                force.x -= repulsive_force.x;
                force.y -= repulsive_force.y;
            }
        }

        //clamp force to 50 -50
        force.x = force.x.clamp(-50.0, 50.0);
        force.y = force.y.clamp(-50.0, 50.0);

        force
    }
}