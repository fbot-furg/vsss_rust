use plotters::prelude::*;
use crate::{TEAM, Vector, Obstacle, Team};
//Unit Vector Field
pub struct UVF {
    de: f64,
    kr: f64,
    d_min: f64,
    delta: f64,
    center_goal: Vector,
    goal: Vector,
    obstacles: Vec<Obstacle>
}

impl UVF {
    pub fn new(goal: Vector, obstacles: Vec<Obstacle>) -> Self {
        let center_goal = match *TEAM {
            Team::Yellow => Vector::new(-75.0 , 0.0),
            Team::Blue => Vector::new(75.0 , 0.0)
        };

        Self {
            de: 10.0,
            kr: 10.0,
            d_min: 10.0,
            delta: 1.0,
            center_goal,
            goal,
            obstacles
        }
    }

    pub fn set_de(&mut self, de: f64) -> &mut Self {
        self.de = de;
        self
    }
    
    pub fn set_kr(&mut self, kr: f64) -> &mut Self {
        self.kr = kr;
        self
    }

    pub fn set_d_min(&mut self, d_min: f64) -> &mut Self {
        self.d_min = d_min;
        self
    }

    pub fn set_delta(&mut self, delta: f64) -> &mut Self {
        self.delta = delta;
        self
    }

    pub fn calculate_force(&self, point: &Vector) -> Vector {
        let angle_to_center_goal = self.goal.orientation_to(&self.center_goal);

        let point_translate = point
            .translate(&self.goal)
            .rotate(-angle_to_center_goal);
            
        let obstacle_field = self.calculate_obstacle_field(&point);
        let locomotion_field = self.calculate_locomotion_field(&point_translate, angle_to_center_goal);

        //get closest obstacle
        let closest_obstacle = self.obstacles.iter().min_by(|a, b| {
            let distance_a = a.point().distance_to(&point);
            let distance_b = b.point().distance_to(&point);

            distance_a.partial_cmp(&distance_b).unwrap()
        });

        let closest_obstacle = match closest_obstacle {
            Some(obstacle) => obstacle,
            None => return locomotion_field
        };

        let distance_to_obstacle = closest_obstacle.point().distance_to(&point);

        let force = if distance_to_obstacle < self.d_min {
            obstacle_field.opposite()
        } else {
            let g = self.gausian(distance_to_obstacle);

            obstacle_field
                .scale(g)
                .add(&locomotion_field.scale(1.0 - g))
        };

        force
    }

    fn gausian(&self, x: f64) -> f64 {
        (-x.powi(2) / (2.0 * self.delta.powi(2))).exp()
    }

    fn calculate_obstacle_field(&self, point: &Vector) -> Vector {
        let mut force_x = 0.0;
        let mut force_y = 0.0;

        for obstacle in &self.obstacles {
            let obstacle_point = obstacle.point();

            let distance_to_obstacle = point.distance_to(&obstacle_point);
            let cosi = (point.x() - obstacle_point.x()) / distance_to_obstacle;
            let sini = (point.y() - obstacle_point.y()) / distance_to_obstacle;
            
            force_x += cosi/distance_to_obstacle;
            force_y += sini/distance_to_obstacle;
        }

        let a = (force_x.powi(2) + force_y.powi(2)).sqrt();

        let auf = (force_y/a).atan2(force_x/a);

        Vector::new(auf.cos(), auf.sin())
    }

    fn calculate_locomotion_field(&self, point: &Vector, fix_angle: f64) -> Vector {       
        let pr = Vector::new(point.x, point.y - self.de);
        let pl = Vector::new(point.x, point.y + self.de);

        let vector = if point.y > 0.0 {
            self.hs_ccw(&pr)
        } else {
            self.hs_cw(&pl)
        };
        
        // let vector =  if point.y >= self.de {
        //     self.hs_ccw(&pr)
        // } else if point.y < -self.de {
        //     self.hs_cw(&pl)
        // } else {
            
        // };

        // let yl = self.de + point.y;
            // let yr = self.de - point.y;

            // let x = (yl * self.hs_ccw(&pl).cos()) + (yr * self.hs_cw(&pr).cos());
            // let y = (yl * self.hs_ccw(&pl).sin()) + (yr * self.hs_cw(&pr).sin());
            
            // let angle = y.atan2(x);

            // Vector::new(angle.cos(), angle.sin())
        
        // vector
        vector.rotate(fix_angle)
    }

    fn hs_cw(&self, point: &Vector) -> Vector {
        let r = (point.x.powi(2) + point.y.powi(2)).sqrt();
        let theta = point.y.atan2(point.x);

        if r > self.de {
            let hs_cw = theta + 90.0_f64.to_radians() * ((self.de + self.kr) / (r + self.kr));

            Vector::new(hs_cw.cos(), hs_cw.sin())
        } else {
            let hs_cw = theta + 90.0_f64.to_radians() * (r / self.de).sqrt();

            Vector::new(hs_cw.cos(), hs_cw.sin())
        }
    }

    fn hs_ccw(&self, point: &Vector) -> Vector {
        let r = (point.x.powi(2) + point.y.powi(2)).sqrt();
        let theta = point.y.atan2(point.x);

        if r > self.de {
            let hs_ccw = theta - 90.0_f64.to_radians() * ((self.de + self.kr) / (r + self.kr));

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
        let name = format!("vector_field_{}_{}.png", self.kr, self.de);

        let root = BitMapBackend::new(&name, (1280, 720)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption("Vector Field", ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-75.0..75.0, -65.0..65.0)?;

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

        for x in (-100..100).step_by(2) {
            for y in (-100..100).step_by(2){
                let point = Vector::new(x as f64, y as f64);
                
                let vector = self.calculate_force(&point);
                vector_field.push((point.x, point.y, -vector.x, -vector.y));
            }
        }

        for (x, y, dx, dy) in vector_field {
            
            chart.draw_series(PointSeries::of_element(
                vec![(x, y)],
                5,
                ShapeStyle::from(&BLACK).filled(),
                &|_, _, style| {
                    let arrow_head = Vector::new(dx, dy);

                    let arrow_head_1 = arrow_head.rotate(135.0_f64.to_radians());
                    let arrow_head_2 = arrow_head.rotate(-135.0_f64.to_radians());

                    let arrow_head_1 = Vector::new(x + arrow_head_1.x, y + arrow_head_1.y);

                    let arrow_head_2 = Vector::new(x + arrow_head_2.x, y + arrow_head_2.y);

                    PathElement::new(vec![
                        (x, y),
                        (x + dx, y + dy),
                        (arrow_head_1.x, arrow_head_1.y),
                        (x + dx, y + dy),
                        (arrow_head_2.x, arrow_head_2.y),
                    ], style)
                },
            ))?;
                
        }

        // chart.draw_series(LineSeries::new(
        //     vec![(x, y), (x + dx, y + dy)],
        //     &RED,
        // ))?;

        Ok(())
    }
}