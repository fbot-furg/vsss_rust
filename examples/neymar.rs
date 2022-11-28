use vsss_rust_client::{FIRASIM};
use fbot_vss_rust::{Origin, Robot, Ball, UVF, Obstacle, Vector, TEAM, Command, Team};

fn main() {
    let team = *TEAM;
    let ball = Ball::new(Origin::FIRASIM);

    let mut goalie = Robot::new(Origin::FIRASIM, 1);

    goalie
        .set_distance_pid((1.0, 0.0, 1.0))
        .set_orientation_pid((25.0, 0.02, 1.0));

    loop {
        let enemy_team = Obstacle::enemy_team_fira();

        let mut uvf = UVF::new(ball.point(), vec![]);

        uvf.set_de(8.0)
            .set_kr(8.0)
            .set_d_min(10.0);

        let force = uvf.calculate_force(&goalie.point());
        let target_point = Vector::new(goalie.point().x() - force.x(), goalie.point().y() - force.y());

        goalie.go_to2(target_point, ball.point());

        FIRASIM.send_command(vec![goalie.get_command()]); 
    }
}