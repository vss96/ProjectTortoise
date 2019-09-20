extern crate time;

use std::borrow::{Borrow, BorrowMut};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use time::PreciseTime;

fn find_file_name(full_line : &String) -> usize {
    let mut counter = 0;
    for ch in full_line.chars().rev(){
        if ch == ':' {
            break;
        }
        counter+=1;
    }
    full_line.len() - counter
}
fn main() {
    let addr = "127.0.0.1:6009";
    let startTime = PreciseTime::now();
    let mut connection = TcpStream::connect(addr).expect("Connection Refused");

    let mut msg_counter = 0;


    let incoming_stream = BufReader::new(&connection);
    for mut line in incoming_stream.lines() {
        let mut entire_line = line.as_ref().unwrap();
        let break_point = find_file_name(entire_line);
        let file_name = &entire_line[break_point..];
        let mut line_string = &entire_line[..break_point-1];
        let mut fresult =
            File::create("../../Files/".to_string() + &file_name);
        match fresult {
            Ok(mut file) => {
                msg_counter = msg_counter + 1;
                file.write_all( line_string.as_bytes());
            }
            Err(err) => {
                println!("Files that error out : {}",&file_name);
                println!("Line String : {} \n Break point : {}",line_string,find_file_name( &String::from(line_string)));
            }
        }
//            println!("Message Name / Message = {} / {}", file_name, line_string);

    }
    println!("Total Messages : {}", msg_counter);
    let total_time = startTime.to(PreciseTime::now()).num_seconds();
    println!(
        "Time Taken : {}, Throughput : {}",
        total_time,
        msg_counter / total_time
    );

}
