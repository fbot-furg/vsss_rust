use vsss_rust_client::{FIRASIM};
use fbot_vss_rust::{Origin, Robot, Ball, UVF, Obstacle, Vector, TEAM, Command, Team};

fn main() {
    let team = *TEAM;
    let ball = Ball::new(Origin::FIRASIM);

    let mut goalie = Robot::new(Origin::FIRASIM, 1);

    goalie
        .set_distance_pid((50.0, 0.002, 1.0))
        .set_orientation_pid((20.0, 0.1, 1.0));

    loop {
        let goal = Vector::new(team.enemy_goal_x(), ball.y());

        goalie.go_to2(goal);

        FIRASIM.send_command(vec![goalie.get_command()]); 
    }
}