extern crate queues;

use queues::*;
use std::borrow::BorrowMut;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::string::String;

pub struct CircularQueue {
    c_queue: Queue<String>,
    c_size: usize,
    write_pointer: File,
    read_pointer: BufReader<File>,
}

pub trait QueueOperations {
    fn set_size(&mut self, c_size: usize);
    fn push_message(&mut self, json_message: String);
    fn get_message(&mut self) -> String;
}

impl Default for CircularQueue {
    fn default() -> CircularQueue {
        CircularQueue {
            c_queue: queue![],
            c_size: 100000,
            write_pointer: File::create("tortoise.log").unwrap(),
            read_pointer: BufReader::new(File::open("tortoise.log").unwrap()),
        }
    }
}

impl QueueOperations for CircularQueue {
    fn set_size(&mut self, c_size: usize) {
        self.c_size = c_size;
    }

    fn push_message(&mut self, json_message: String) {
        if self.c_queue.size() == self.c_size {
            let json = self.c_queue.peek().unwrap();
            self.save_in_file(json);
            self.c_queue.remove();
            self.c_queue.add(json_message);
        } else {
            self.c_queue.add(json_message);
        }
    }

    fn get_message(&mut self) -> String {
        let mut message = self.c_queue.peek().unwrap_or_default();
        self.c_queue.remove();
        self.pull_message();
        return String::from(message.trim_end_matches("\n"));
    }
}

impl CircularQueue {
    fn save_in_file(&mut self, json_message: String) {
        let mut file = BufWriter::new(self.write_pointer.borrow_mut());
        file.write_all((json_message + "\n").as_bytes())
            .expect("Unable to write into the log file");
    }

    fn pull_message(&mut self) {
        //Does not remove lines, will have to look into it.
        let reader = self.read_pointer.borrow_mut();
        let mut line_string: String = String::from("");
        reader.read_line(line_string.borrow_mut()).unwrap();
        if !line_string.is_empty() {
            self.c_queue.add(line_string);
        }
    }
}

#[test]
fn circular_queue_test() {
    let mut _cqueue: CircularQueue = Default::default();
    assert_eq!(_cqueue.c_size, 100000);
    _cqueue.set_size(100);
    assert_eq!(_cqueue.c_size, 100);

    _cqueue.push(String::from("Hello"));
    assert_eq!(_cqueue.c_queue.peek().unwrap(), "Hello");
}
