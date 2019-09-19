extern crate time;

use std::borrow::{Borrow, BorrowMut};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use time::PreciseTime;

trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> Self;
}

impl StringUtils for String {
    fn substring(&self, start: usize, len: usize) -> Self {
        self.chars().skip(start).take(len).collect()
    }
}

fn find_file_name(full_name : &String) -> usize{
    let mut counter = 0;
    for ch in full_name.chars().rev(){
        if ch == ':'{
            break;
        }
        counter+=1;
    }
    full_name.len() - counter
}


fn main() {
    let addr = "127.0.0.1:6007";
    let startTime = PreciseTime::now();
    let mut connection = TcpStream::connect(addr).expect("Connection Refused");

    let mut msg_counter = 0;


        let incoming_stream = BufReader::new(&connection);
        for line in incoming_stream.lines() {
            let mut entire_line = line.as_ref().unwrap();
            let break_point = find_file_name(entire_line);
            let file_name = entire_line.substring(break_point,entire_line.len());
            let line_string = line.unwrap().substring(0,break_point-1);
            let mut file =
                File::create("Files/".to_string() + &file_name)
                    .unwrap();
            msg_counter = msg_counter + 1;
            file.write_all( line_string.as_bytes());
        }
        println!("Total Messages : {}", msg_counter);
        let total_time = startTime.to(PreciseTime::now()).num_seconds();
        println!(
            "Time Taken : {}, Throughput : {}",
            total_time,
            msg_counter / total_time
        );

}
