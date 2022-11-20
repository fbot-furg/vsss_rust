use crate::{Point, Goal, Obstacle};
use plotters::prelude::*;

//TODO
// plot vector field


//Unit Vector Field
pub struct Uvf {
    goal: Goal,
    obstacles: Vec<Obstacle>
}

impl Uvf {
    pub fn new(goal: Goal, obstacles: Vec<Obstacle>) -> Self {
        Self {
            goal: goal,
            obstacles: obstacles
        }
    }

    pub fn calculate_vector(&self, point: Point) -> Point {
        let locomotion_field = self.calculate_locomotion_field(point);
    }

    fn calculate_locomotion_field(&self, point: Point) -> Point {
        //Using hyperbolic spiral equation
        // let r = 
    }

    pub fn plot_vector_field(&self) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new("vector_field.png", (640, 480)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption("Vector Field", ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-10.0..10.0, -10.0..10.0)?;

        chart.configure_mesh().draw()?;

        Ok(())
    }
}