use std::f32::consts::E;

use vsss_rust_client::{FIRASIM};
use fbot_vss_rust::{Origin, Robot, Ball, UVF, Obstacle, Vector, TEAM, Command, Team};

fn main() {
    let team = *TEAM;
    let ball = Ball::new(Origin::FIRASIM);

    let mut goalie = Robot::new(Origin::FIRASIM, 0);
    let mut neymar = Robot::new(Origin::FIRASIM, 1);
    

    goalie
        .set_distance_pid((1000.0, 0.0, 1.0))
        .set_orientation_pid((40.0, 0.0, 1.0));

    neymar
        .set_distance_pid((35.0, 0.0, 1000.0))
        .set_orientation_pid((15.0, 0.4, 8000.0));

    loop {
        let mut enemy_team = Obstacle::enemy_team_fira();
        enemy_team.push(Obstacle::new(goalie.point(), 0.5));

        let goal = Vector::new(team.enemy_goal_x(), ball.y().clamp(-20.0, 20.0));

        
        let mut uvf = UVF::new(ball.point(), enemy_team);

        uvf.set_de(12.0)
            .set_kr(5.0)
            .set_d_min(10.0);

        let force = uvf.calculate_force(&neymar.point());
        let target_point = Vector::new(neymar.point().x() - force.x(), neymar.point().y() - force.y());

        neymar.go_to_attack(target_point, ball.point());
        goalie.go_to2(goal);

        // println!("neymar: {:?}", neymar.get_command());

        FIRASIM.send_command(vec![neymar.get_command(), goalie.get_command()]); 
    }
}