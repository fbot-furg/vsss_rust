use plotters::prelude::*;
use crate::{Vector, Obstacle, Team};
//Unit Vector Field
pub struct UVF {
    de: f64,
    kr: f64,
    center_goal: Vector,
    goal: Vector,
    obstacles: Vec<Obstacle>
}

impl UVF {
    pub fn new(goal: Vector, obstacles: Vec<Obstacle>, team: Team) -> Self {
        let center_goal = match team {
            Team::Yellow => Vector::new(70.0 , 0.0),
            Team::Blue => Vector::new(-70.0 , 0.0)
        };

        Self {
            de: 8.0,
            kr: 3.0,
            center_goal,
            goal,
            obstacles
        }
    }

    pub fn calculate_force(&self, point: &Vector) -> Vector {
        let angle_to_center_goal = self.goal.orientation_to(&self.center_goal);

        let point = point
            .translate(&self.goal)
            .rotate(-angle_to_center_goal);
            

        let locomotion_field = self.calculate_locomotion_field(&point, angle_to_center_goal);

        locomotion_field
    }

    fn calculate_locomotion_field(&self, point: &Vector, fix_angle: f64) -> Vector {       
        let pr = Vector::new(point.x, point.y - self.de);
        let pl = Vector::new(point.x, point.y + self.de);
        
        let vector =  if point.y > self.de {
            self.hs_ccw(&pr)
        } else if point.y < -self.de {
            self.hs_cw(&pl)
        } else {
            let cos = (self.de - point.y).abs() * self.hs_cw(&pl).cos() + (self.de + point.y).abs() * self.hs_ccw(&pr).cos();
            let sin = (self.de - point.y).abs() * self.hs_cw(&pl).sin() + (self.de + point.y).abs() * self.hs_ccw(&pr).sin();

            let tuf = sin.atan2(cos);

            Vector::new(tuf.cos(), tuf.sin())
        };

        vector.rotate(fix_angle)
    }

    fn hs_cw(&self, point: &Vector) -> Vector {
        let r = (point.x.powi(2) + point.y.powi(2)).sqrt();
        let theta = point.y.atan2(point.x);

        if r > self.de {
            let hs_cw = theta - 90.0_f64.to_radians() * ((self.de + self.kr) / (r + self.kr));

            Vector {
                x: hs_cw.cos(),
                y: hs_cw.sin()
            }
        } else {
            let hs_cw = theta + 90.0_f64.to_radians() * (r / self.de).sqrt();

            Vector {
                x: hs_cw.cos(),
                y: hs_cw.sin()
            }
        }
    }

    fn hs_ccw(&self, point: &Vector) -> Vector {
        let r = (point.x.powi(2) + point.y.powi(2)).sqrt();
        let theta = point.y.atan2(point.x);

        if r > self.de {
            let hs_ccw = theta + 90.0_f64.to_radians() * ((self.de + self.kr) / (r + self.kr));

            Vector {
                x: hs_ccw.cos(),
                y: hs_ccw.sin()
            }
        } else {
            let hs_ccw = theta - 90.0_f64.to_radians() * (r / self.de).sqrt();

            Vector {
                x: hs_ccw.cos(),
                y: hs_ccw.sin()
            }
        }
    }

    pub fn plot_vector_field(&self) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new("vector_field.png", (640, 480)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption("Vector Field", ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-50.0..50.0, -50.0..50.0)?;

        chart.configure_mesh().draw()?;

        //draw goal
        chart.draw_series(PointSeries::of_element(
            vec![(self.goal.x, self.goal.y)],
            5,
            ShapeStyle::from(&RED).filled(),
            &|coord, size, style| {
                EmptyElement::at(coord) + Circle::new((0, 0), size, style)
            },
        ))?;

        //draw obstacles
        for obstacle in &self.obstacles {
            chart.draw_series(PointSeries::of_element(
                vec![(obstacle.point.x, obstacle.point.y)],
                5,
                ShapeStyle::from(&BLACK).filled(),
                &|coord, size, style| {
                    EmptyElement::at(coord) + Circle::new((0, 0), size, style)
                },
            ))?;
        }

        //calculate vector field
        let mut vector_field = Vec::new();
        for x in -50..50 {
            for y in -50..50{
                let point = Vector::new(x as f64, y as f64);
                
                let vector = self.calculate_force(&point);
                vector_field.push((point.x, point.y, vector.x, vector.y));
            }
        }

        //Draw vector field with LineSeries
        for (x, y, dx, dy) in vector_field {
            chart.draw_series(LineSeries::new(
                vec![(x, y), (x + dx, y + dy)],
                &RED,
            ))?;
        }

        Ok(())
    }
}