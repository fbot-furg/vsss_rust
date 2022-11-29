use vsss_rust_client::{FIRASIM};
use fbot_vss_rust::{Origin, Robot, Ball, UVF, Obstacle, Vector, TEAM, Command, Team};

fn main() {
    let team = *TEAM;
    let ball = Ball::new(Origin::FIRASIM);

    let mut goalie = Robot::new(Origin::FIRASIM, 1);

    goalie
        .set_distance_pid((30.0, 0.0, 1000.0))
        .set_orientation_pid((15.0, 0.04, 8000.0));

    loop {
        let enemy_team = Obstacle::enemy_team_fira();

        let mut uvf = UVF::new(ball.point(), enemy_team);

        uvf.set_de(12.0)
            .set_kr(10.0)
            .set_d_min(10.0);

        let force = uvf.calculate_force(&goalie.point());
        let target_point = Vector::new(goalie.point().x() - force.x(), goalie.point().y() - force.y());

        goalie.go_to_attack(target_point, ball.point());

        FIRASIM.send_command(vec![goalie.get_command()]); 
    }
}