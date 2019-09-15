extern crate time;

use std::borrow::{Borrow, BorrowMut};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use time::PreciseTime;

fn main() {
    let addr = "127.0.0.1:6007";
    let startTime = PreciseTime::now();
    let mut connection = TcpStream::connect(addr).expect("Connection Refused");

    let mut msg_counter = 0;
    let incoming_stream = BufReader::new(connection);
    for line in incoming_stream.lines() {
        let mut file =
            File::create("../../Documents/Files/".to_string() + msg_counter.to_string().as_ref() + ".json")
                .unwrap();
        msg_counter = msg_counter + 1;
        //       println!(" Message: {}", line.unwrap());
        file.write_all(line.unwrap().as_bytes());
    }
    println!("Total Messages : {}", msg_counter);
    let total_time = startTime.to(PreciseTime::now()).num_seconds();
    println!(
        "Time Taken : {}, Throughput : {}",
        total_time,
        msg_counter / total_time
    );
}
