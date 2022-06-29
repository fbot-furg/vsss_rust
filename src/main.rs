use fbot_vss_rust::{fira, deserialize};

fn main() {
    let array: [u8; 382] = [8, 192, 184, 3, 18, 209, 2, 10, 27, 9, 77, 197, 77, 131, 58, 6, 189, 63, 17, 64, 17, 224, 58, 100, 10, 146, 191, 25, 129, 196, 113, 147, 24, 4, 150, 63, 18, 54, 17, 83, 31, 191, 202, 77, 96, 225, 191, 25, 111, 214, 1, 163, 200, 19, 226, 191, 33, 198, 214, 82, 191, 55, 109, 102, 63, 41, 248, 166, 72, 121, 92, 50, 242, 60, 49, 91, 222, 44, 247, 27, 147, 191, 188, 57, 31, 70, 180, 43, 231, 148, 237, 188, 18, 47, 8, 1, 17, 254, 255, 255, 255, 229, 221, 172, 63, 25, 255, 255, 255, 159, 64, 82, 221, 191, 41, 247, 80, 240, 210, 97, 76, 148, 60, 49, 172, 185, 142, 38, 198, 33, 235, 188, 57, 8, 80, 11, 213, 179, 150, 108, 188, 18, 47, 8, 2, 17, 1, 0, 0, 64, 17, 12, 225, 63, 25, 0, 0, 0, 64, 95, 89, 216, 63, 41, 63, 166, 1, 250, 14, 241, 254, 188, 49, 235, 198, 147, 179, 98, 181, 196, 188, 57, 172, 216, 31, 253, 30, 74, 140, 60, 26, 54, 17, 176, 77, 65, 85, 212, 169, 225, 191, 25, 12, 7, 37, 179, 204, 200, 221, 191, 33, 193, 153, 24, 134, 66, 109, 102, 63, 41, 255, 110, 110, 51, 75, 95, 255, 188, 49, 86, 82, 94, 214, 66, 63, 254, 188, 57, 89, 224, 169, 137, 234, 185, 220, 188, 26, 47, 8, 1, 17, 14, 0, 0, 0, 122, 209, 179, 191, 25, 2, 0, 0, 128, 171, 166, 192, 63, 41, 100, 178, 88, 101, 155, 202, 146, 188, 49, 164, 10, 182, 142, 48, 29, 195, 188, 57, 217, 29, 147, 125, 64, 128, 130, 60, 26, 47, 8, 2, 17, 252, 255, 255, 255, 117, 53, 208, 63, 25, 0, 0, 0, 160, 130, 147, 224, 63, 41, 1, 108, 47, 198, 78, 114, 243, 60, 49, 251, 134, 126, 0, 132, 88, 186, 188, 57, 192, 209, 152, 142, 14, 119, 248, 59, 26, 36, 9, 205, 204, 204, 204, 204, 204, 244, 63, 17, 0, 0, 0, 0, 0, 0, 248, 63, 25, 154, 153, 153, 153, 153, 153, 217, 63, 33, 154, 153, 153, 153, 153, 153, 185, 63];

    let common = deserialize(&array).unwrap();

    println!("{:?}", common);


    // let array_teste = [10, 36, 9, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 18, 33, 9, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 40, 0, 48, 0];
    // let array_teste2 = [10, 36, 9, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 18, 33, 9, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 40, 0, 48, 0, 18, 33, 9, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 40, 0, 48, 0, 18, 33, 9, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 40, 0, 48, 0, 18, 33, 9, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 40, 1, 48, 0, 18, 33, 9, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 40, 1, 48, 0, 18, 33, 9, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 40, 1, 48, 0];

    // let packet = match deserialize_shirt(&array) {
    //     Ok(packet) => packet,
    //     Err(e) => {
    //         println!("Error: {:?}", e);
    //         return;
    //     }
    // };

    // println!("{:?}", packet);

    // let teste = fira::GrSimReplacement {
    //     ball: Some(fira::GrSimBallReplacement {
    //         x: Some(0.0),
    //         y: Some(0.0),
    //         vx: Some(0.0),
    //         vy: Some(0.0),
    //     }),
    //     robots: vec![
    //         fira::GrSimRobotReplacement {
    //             id: 0,
    //             x: 0.0,
    //             y: 0.0,
    //             dir: 0.0,
    //             yellowteam: false,
    //             turnon: Some(false),
    //         },
    //         fira::GrSimRobotReplacement {
    //             id: 0,
    //             x: 0.0,
    //             y: 0.0,
    //             dir: 0.0,
    //             yellowteam: false,
    //             turnon: Some(false),
    //         },
    //         fira::GrSimRobotReplacement {
    //             id: 0,
    //             x: 0.0,
    //             y: 0.0,
    //             dir: 0.0,
    //             yellowteam: false,
    //             turnon: Some(false),
    //         },

    //         fira::GrSimRobotReplacement {
    //             id: 0,
    //             x: 0.0,
    //             y: 0.0,
    //             dir: 0.0,
    //             yellowteam: true,
    //             turnon: Some(false),
    //         },
    //         fira::GrSimRobotReplacement {
    //             id: 0,
    //             x: 0.0,
    //             y: 0.0,
    //             dir: 0.0,
    //             yellowteam: true,
    //             turnon: Some(false),
    //         },
    //         fira::GrSimRobotReplacement {
    //             id: 0,
    //             x: 0.0,
    //             y: 0.0,
    //             dir: 0.0,
    //             yellowteam: true,
    //             turnon: Some(false),
    //         },
    //     ]
    // };

    // let packet = fira::GrSimPacket {
    //     replacement: Some(teste),
    //     commands: vec![
    //         fira::GrSimCommand {
    //             id: 0,
    //             vx: 0.0,
    //             vy: 0.0,
    //             w: 0.0,
    // }

    // let binary = serialize_replacement(&teste);

    // println!("{:?}", binary);
    

}