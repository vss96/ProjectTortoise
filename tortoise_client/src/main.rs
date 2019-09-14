extern crate time;

use std::borrow::{Borrow, BorrowMut};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use time::PreciseTime;

fn main() {
    let addr = "192.168.1.80:6006";
    let startTime = PreciseTime::now();
    let mut connection = TcpStream::connect(addr).expect("Connection Refused");

    let mut msg_counter = 0;
    let mut stuff = Vec::<u8>::new();
    let pepeg = BufReader::new(connection);
    for line in pepeg.lines() {
        let mut file = BufWriter::new(
            File::create("Files/".to_string() + msg_counter.to_string().as_ref()).unwrap(),
        );
        msg_counter = msg_counter + 1;
        //       println!(" Message: {}", line.unwrap());
        file.write_all(line.unwrap().as_bytes());
    }
    println!("Total Messages : {}", msg_counter);
    let totalTime = startTime.to(PreciseTime::now()).num_seconds();
    println!(
        "Time Taken : {}, Throughput : {}",
        totalTime,
        msg_counter / totalTime
    );
}
