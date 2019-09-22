extern crate time;

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use time::PreciseTime;

fn find_file_name(full_line: &String) -> usize {
    let mut counter = 0;
    for ch in full_line.chars().rev() {
        if ch == ':' {
            break;
        }
        counter += 1;
    }
    full_line.len() - counter
}

fn spawn_consumer(port: &i32) {
    let addr = "52.3.229.162:".to_owned() + &port.to_string();
    let connection = TcpStream::connect(addr).expect("Connection Refused");
    let mut msg_counter = 0;
    let incoming_stream = BufReader::with_capacity(200000, &connection);
    let consumer_start_time = PreciseTime::now();
    let mut total_time = 0;
    for line in incoming_stream.lines() {
        let entire_line = line.as_ref().unwrap();
        let break_point = find_file_name(entire_line);
        let file_name = &entire_line[break_point..];
        let line_string = &entire_line[..break_point - 1];
        let fresult =
            File::create("/data/Files/".to_string() + &file_name + ".json");
        match fresult {
            Ok(mut file) => {
                msg_counter = msg_counter + 1;
                file.write_all(line_string.as_bytes()).unwrap_or_default();
                total_time = consumer_start_time.to(PreciseTime::now()).num_milliseconds();
                if msg_counter % 100000 == 0 {
                    println!("Message #{} received / Total Write Time : {}", msg_counter, total_time);
                }
            }
            Err(_err) => {
                println!("Files that error out : {}", &file_name);
                println!("Line String : {} \n Break point : {}", line_string, find_file_name(&String::from(line_string)));
            }
        }
    }
    println!("Total Messages : {}", msg_counter);
    total_time = total_time / 1000;
    println!(
        "Time Taken : {}, Throughput : {}",
        total_time,
        msg_counter / total_time
    );
}

fn main() {
    spawn_consumer(&6881);
}

