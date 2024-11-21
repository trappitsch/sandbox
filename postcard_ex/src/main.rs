//! Test the postcard library
//!
//! Attention: Not compatible with heapless 0.8, use 0.7 instead.

use core::ops::Deref;
use heapless::Vec;

use postcard::{from_bytes, to_vec};

mod commands;
use crate::commands::*;

fn main() {
    let position = Position { x: 123, y: 456 };
    let send_position = Commands::SetPosition(position);

    let send_position_pc: Vec<u8, 32> = to_vec(&send_position).unwrap();
    decode(send_position_pc.deref());

    let query_position = Commands::QueryPosition;

    let query_position_pc: Vec<u8, 32> = to_vec(&query_position).unwrap();
    decode(query_position_pc.deref());

    let send_time = Commands::SetTime(1234567890);
    let send_time_pc: Vec<u8, 32> = to_vec(&send_time).unwrap();
    decode(send_time_pc.deref());

    // Test invalid data - should NOT panic!
    decode(&[0x08, 0x00, 0x00, 0x00, 0x00]);
}

fn decode(bytes: &[u8]) {
    let cmd: Commands = from_bytes(bytes).unwrap_or(Commands::Unknown);

    println!("Bytes{:?}", bytes);

    match cmd {
        Commands::SetPosition(pos) => {
            println!("SetPosition: {:?}", pos);
        }
        Commands::SetTime(time) => {
            println!("SetTime: {:?}", time);
        }
        Commands::QueryPosition => {
            println!("Query position");
        }
        Commands::Unknown => {
            println!("Unknown command");
        }
    }
}
