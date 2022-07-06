// use tokio::net::UdpSocket;
// use tokio::time::{Duration, sleep};

// // use std::sync::{Arc, Mutex};
// // use bytes::Bytes;


// use fbot_vss_rust::{fira_protos, deserialize_env, serialize_packet};
// const VISION_ADDRS: &str = "224.0.0.1:10002";
// const COMMAND_ADDRS: &str = "127.0.0.1:20011";

// // type Db = Arc<Mutex<HashMap<String, Bytes>>>;

// #[tokio::main]
// async fn main() {
//     let socket = UdpSocket::bind(VISION_ADDRS).await.unwrap();

//     println!("Listening to {}", VISION_ADDRS);

//     let packet = fira_protos::Packet {
//         cmd: Some(fira_protos::Commands {
//             robot_commands: vec![
//                 fira_protos::Command {
//                     id: 0,
//                     yellowteam: true,
//                     wheel_left: 1.0,
//                     wheel_right: -1.0
//                 },
//                 fira_protos::Command {
//                     id: 1,
//                     yellowteam: true,
//                     wheel_left: 1.0,
//                     wheel_right: -1.0
//                 },
//                 fira_protos::Command {
//                     id: 2,
//                     yellowteam: true,
//                     wheel_left: 1.0,
//                     wheel_right: -1.0
//                 }
//             ]
//         }),
//         replace: None        
//     };
//     let buf_command = serialize_packet(&packet);

//     socket.connect(COMMAND_ADDRS).await.expect("connect function failed");
//     loop {        
//         socket.send_to(&buf_command, COMMAND_ADDRS).await.expect("send erro");
//         println!("Send");

//         // let mut buf = [0; 1024];

//         // let (len, addr) = socket.recv_from(&mut buf).await.unwrap();
//         // println!("{:?} bytes received from {:?}", len, addr);
        

//         // let env = deserialize_env(&buf[..len]).unwrap();

//         // if let Some(frame) = env.frame {
//         //     println!("{:?}", frame.ball);    
//         // }



//         sleep(Duration::from_secs(1)).await;
//     }
// }

use std::net::UdpSocket;
use std::time::Duration;
use std::thread::sleep;

// use std::sync::{Arc, Mutex};
// use bytes::Bytes;


use fbot_vss_rust::{fira_protos, deserialize_env, serialize_packet};
const VISION_ADDRS: &str = "244.0.0.1:10002";
const COMMAND_ADDRS: &str = "127.0.0.1:20011";

// type Db = Arc<Mutex<HashMap<String, Bytes>>>;

// #[tokio::main]
fn main() {
    let socket = UdpSocket::bind(VISION_ADDRS).unwrap();
    println!("Listening to {}", VISION_ADDRS);

    let packet = fira_protos::Packet {
        cmd: Some(fira_protos::Commands {
            robot_commands: vec![
                fira_protos::Command {
                    id: 0,
                    yellowteam: true,
                    wheel_left: 1.0,
                    wheel_right: -1.0
                },
                fira_protos::Command {
                    id: 1,
                    yellowteam: true,
                    wheel_left: 1.0,
                    wheel_right: -1.0
                },
                fira_protos::Command {
                    id: 2,
                    yellowteam: true,
                    wheel_left: 1.0,
                    wheel_right: -1.0
                }
            ]
        }),
        replace: None        
    };
    let buf_command = serialize_packet(&packet);

    // socket.connect(COMMAND_ADDRS).expect("connect function failed");
    loop {        
        socket.send_to(&buf_command, COMMAND_ADDRS).expect("send erro");
        println!("Send {:?}", a);

        let mut buf = [0; 1024];

        let (len, addr) = socket.recv_from(&mut buf).unwrap();
        println!("{:?} bytes received from {:?}", len, addr);
        

        let env = deserialize_env(&buf[..len]).unwrap();

        if let Some(frame) = env.frame {
            println!("{:?}", frame.ball);    
        }



        sleep(Duration::from_millis(100));
    }
}