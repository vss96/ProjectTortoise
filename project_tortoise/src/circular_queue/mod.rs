extern crate queues;

use crossbeam_queue::ArrayQueue;
use std::borrow::BorrowMut;
use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::string::String;

pub struct CircularQueue {
    c_queue: ArrayQueue<String>,
    c_size: usize,
    write_pointer: File,
    read_pointer: BufReader<File>,
}

pub trait QueueOperations {
    fn set_size(&mut self, c_size: usize);
    fn push(&mut self, json_message: String);
    fn pull(&mut self) -> String;
}

impl Default for CircularQueue {
    fn default() -> CircularQueue {
        CircularQueue {
            c_queue: ArrayQueue::<String>::new(100000),
            c_size: 100000,
            write_pointer: File::create("tortoise.log").unwrap(),
            read_pointer: BufReader::new(File::open("tortoise.log").unwrap()),
        }
    }
}

impl QueueOperations for CircularQueue {
    fn set_size(&mut self, c_size: usize) {
        self.c_size = min(c_size, 100000);
    }

    fn push(&mut self, json_message: String) {
        if self.c_queue.capacity() == self.c_size {
            self.save_in_file(json_message);
        } else {
            self.c_queue.push(json_message);
        }
    }

    fn pull(&mut self) -> String {
        let mut message = self.c_queue.pop().unwrap_or_default();
        self.get_file_message();
        return String::from(message.trim_end_matches("\n"));
    }
}

impl CircularQueue {
    fn save_in_file(&mut self, json_message: String) {
        let mut file = BufWriter::new(self.write_pointer.borrow_mut());
        file.write_all((json_message + "\n").as_bytes())
            .expect("Unable to write into the log file");
    }

    fn get_file_message(&mut self) {
        //Does not remove lines, will have to look into it.
        let reader = self.read_pointer.borrow_mut();
        let mut line_string: String = String::from("");
        reader.read_line(line_string.borrow_mut()).unwrap();
        if !line_string.is_empty() {
            self.c_queue.push(line_string);
        }
    }
}

#[test]
fn default_test() {
    let mut _cqueue: CircularQueue = Default::default();
    assert_eq!(_cqueue.c_size, 100000);
    _cqueue.set_size(100);
    assert_eq!(_cqueue.c_size, 100);

    _cqueue.push(String::from("Hello"));
    assert_eq!(_cqueue.pull(), "Hello");
}

#[test]
fn operation_test() {
    let mut _cqueue = CircularQueue {
        write_pointer: File::create("tortoise_test.log").unwrap(),
        read_pointer: BufReader::new(File::open("tortoise_test.log").unwrap()),
        ..Default::default()
    };
    _cqueue.set_size(1);
    _cqueue.push("1".to_string());
    _cqueue.push("2".to_string());
    _cqueue.push("3".to_string());
    _cqueue.push("4".to_string());
    _cqueue.push("5".to_string());
    assert_eq!("1", _cqueue.pull());
    _cqueue.push("6".to_string());
    assert_eq!("2", _cqueue.pull());
    assert_eq!("3", _cqueue.pull());
    assert_eq!("4", _cqueue.pull());
    assert_eq!("5", _cqueue.pull());
    assert_eq!("6", _cqueue.pull());
}
