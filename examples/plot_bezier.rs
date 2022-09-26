// use fbot_vss_rust::fbot_fira::{fira_protos, get_ball, get_yellow_robot, Robot, Team};
// use flo_curves::{bezier::Curve, Coord2, BezierCurve};

// use plotlib::page::Page;
// use plotlib::repr::Plot;
// use plotlib::view::ContinuousView;
// use plotlib::style::{PointMarker, PointStyle};

// fn main() {
//     let goalie = Robot::new(0, Team::Yellow);
//     let ball = get_ball();

//     let goalie_cp = goalie.control_point();
    
    
//     println!("{:?}", goalie_cp);

//     let bezier = Curve {
//         start_point: Coord2 (goalie.x(), goalie.y()),
//         end_point: Coord2 (ball.x, ball.y),
//         control_points: (Coord2 (0.0, 0.0), Coord2 (0.0, 0.0) )
//     };

//     plot(&bezier);

//     // for i in 0..20 { 

//     //     let pos: f64 = i as f64 / 20.0;
        
//     //     let point = bezier.point_at_pos(pos);
//     //     let (x, y) = (point.0, point.1);

//     //     goalie.go_to(x, y);

//     // }
// }

// fn plot (bezier: &Curve) {
//     let a = vec![
//         (bezier.point_at_pos(0.0).0, bezier.point_at_pos(0.0).1),
//         (bezier.point_at_pos(0.2).0, bezier.point_at_pos(0.2).1),
//         (bezier.point_at_pos(0.4).0, bezier.point_at_pos(0.4).1),
//         (bezier.point_at_pos(0.6).0, bezier.point_at_pos(0.6).1),
//         (bezier.point_at_pos(0.7).0, bezier.point_at_pos(0.7).1),
//         (bezier.point_at_pos(0.8).0, bezier.point_at_pos(0.8).1),
//         (bezier.point_at_pos(1.0).0, bezier.point_at_pos(1.0).1),
//     ];

//     println!("{:?}", a);

//     let s1: Plot = Plot::new(a).point_style(
//         PointStyle::new()
//             .marker(PointMarker::Square) // setting the marker to be a square
//             .colour("#DD3355"),
//     ); // and a custom colour;

//     let v = ContinuousView::new()
//         .add(s1)
//         .x_range(-0.75, 0.75)
//         .y_range(-0.65, 0.65)
//         .x_label("X")
//         .y_label("Y");

//     // A page with a single view is then saved to an SVG file
//     Page::single(&v).save("bezier.svg").unwrap();
// }
fn main() {
    
}