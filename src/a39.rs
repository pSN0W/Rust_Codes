// Topic: Channels
//
// Summary:
//   Using the existing code, create a program that simulates an internet-of-things
//   remote control light bulb. The color of the light can be changed remotely.
//   Use threads and channels to communicate what color the light bulb should display.
//
// Requirements:
// * Create a separate thread representing the light bulb
// * Use a channel to communicate with the thread
// * Display a color change message using the println! macro
// * The light bulb must also be able to turn on and off
//   * Display whether the light is on or off on each color change
// * Turn off the light when disconnecting from it
//
// Notes:
// * Remember to add `crossbeam-channel` to your Cargo.toml file
// * Use the `colored` crate if you want to get fancy and display actual colors
// * The docs.rs site can be used to read documentation for third-party crates
// * Disconnection can be accomplished by dropping the sender, or
//   by telling the thread to self-terminate
// * Use `cargo test --bin a39` to test your program to ensure all cases are covered

use crossbeam_channel::{unbounded, Receiver};
use std::thread::{self, JoinHandle};
use colored::*;

enum LightMsg {
    // Add additional variants needed to complete the exercise
    ChangeColor(u8, u8, u8),
    Disconnect,
    On,
    Off,
}

#[derive(Debug)]
enum LightStatus {
    Off,
    On,
}

// The function gets reciever as an argument which recieve data of type LightMsg
// The function returns a handler to join the thread along with data as LightStatus
fn spawn_light_thread(receiver: Receiver<LightMsg>) -> JoinHandle<LightStatus> {
    // creating a new thread 
    // The return value of the closure will be returned with the Join =Handler
    thread::spawn(move || {
        let mut status = LightStatus::Off;
        loop {
            match receiver.recv() {
                Ok(msg) => match msg {
                    LightMsg::On => {
                        println!("Lights turned on");
                        status = LightStatus::On;
                    },
                    LightMsg::Off => {
                        println!("Lights turned off");
                        status = LightStatus::Off;
                    },
                    LightMsg::ChangeColor(r,g,b) => {
                        println!("Lights changed to : {}","      ".on_truecolor(r,g,b));
                    },
                    LightMsg::Disconnect => {
                        println!("Disconnecting.....");
                        status = LightStatus::Off;
                        break;
                    }
                }
                Err(e) => {
                    status = LightStatus::Off;
                    println!("Worker Disconnected");
                    break;
                }
            }
        }
        status
    })
}

fn main() {
    // Creating a channel 
    let (s,r) = unbounded();

    // create a new thread with the reciever
    let reciever_handler = spawn_light_thread(r);
    s.send(LightMsg::On);
    s.send(LightMsg::ChangeColor(255,0,0));
    s.send(LightMsg::ChangeColor(0,255,0));
    s.send(LightMsg::ChangeColor(0,0,255));
    s.send(LightMsg::Off);
    s.send(LightMsg::Disconnect);

    // The message send by the reciever
    let status = reciever_handler.join();

    println!("{:?}",status);
}

#[cfg(test)]
mod test {
    use super::*;
    use crossbeam_channel::unbounded;

    #[test]
    fn light_off_when_disconnect() {
        let (s, r) = unbounded();

        let light = spawn_light_thread(r);
        s.send(LightMsg::Disconnect).expect("channel disconnected");

        let light_status = light.join().expect("failed to join light thread");

        if let LightStatus::On = light_status {
            panic!("light should be off after disconnection");
        }
    }

    #[test]
    fn light_off_when_dropped() {
        let (s, r) = unbounded();

        let light = spawn_light_thread(r);
        drop(s);

        let light_status = light.join().expect("failed to join light thread");

        if let LightStatus::On = light_status {
            panic!("light should be off after dropping sender");
        }
    }
}
