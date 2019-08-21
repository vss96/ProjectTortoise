extern crate queues;

use queues::*;
use std::fs::File;
use std::io::{Write, BufWriter, BufReader, BufRead};
use std::string::String;
use std::borrow::BorrowMut;
use std::f32::consts::E;

pub struct CircularQueue {
    c_queue: Queue<String>,
    c_size: usize,
    file_pointer: File,
}

pub trait QueueOperations {
    fn set_size(self: &mut Self, c_size: usize);
    fn push(self: &mut Self, json_message: String);
    fn pull(self: &mut Self);
}

impl Default for CircularQueue {
     fn default() -> CircularQueue {
        CircularQueue {
            c_queue: queue![],
            c_size: 100000,
            file_pointer: File::create("tortoise.log").unwrap(),
        }
    }
}

impl QueueOperations for CircularQueue {
    fn set_size(self: &mut Self, c_size: usize) {
        self.c_size = c_size;
    }

    fn push(self: &mut Self, json_message: String) {
        if self.c_queue.size() == self.c_size {
            let json = self.c_queue.peek().unwrap();
            self.save_in_file(json);
            self.c_queue.remove();
            self.c_queue.add(json_message);
        } else {
            self.c_queue.add(json_message);
        }
    }

    fn pull(self: &mut Self) {
        //Does not remove lines, will have to look into it.
        let file = BufReader::new(self.file_pointer.borrow_mut());
        for line in file.lines() {
            if self.c_queue.size() < self.c_size {
                self.c_queue.add(line.unwrap());
            } else {
                break;
            }
        }
    }
}

impl CircularQueue {
    fn save_in_file(self: &mut Self, json_message: String) {
        let mut file = BufWriter::new(self.file_pointer.borrow_mut());
        file.write_all(json_message.as_bytes()).expect("Unable to write into the log file");
    }
}

