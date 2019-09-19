extern crate queues;

use std::borrow::{Borrow, BorrowMut};
use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::string::String;
use std::sync::{Arc, Condvar, Mutex};

use crossbeam_queue::ArrayQueue;

pub struct CircularQueue {
    c_queue: ArrayQueue<String>,
    c_size: usize,
    write_pointer: File,
    read_pointer: BufReader<File>,
}

pub struct Queue {
    name: String,
    size: usize,
    _canPush: Arc<(Mutex<bool>, Condvar)>,
    _canPull: Arc<(Mutex<bool>, Condvar)>,
    _queue: ArrayQueue<Vec<u8>>,
}

impl Default for Queue {
    fn default() -> Self {
        Queue {
            name: String::from("Default"),
            size: 500000,
            _canPush: Arc::new((Mutex::new(true), Condvar::new())),
            _canPull: Arc::new((Mutex::new(false), Condvar::new())),
            _queue: ArrayQueue::new(500000),
        }
    }
}

impl QueueOperations<Vec<u8>> for Queue {
    fn push(&self, json_message: Vec<u8>) {
        {
        let isFull = self._queue.is_full();
        let lock_push: &std::sync::Mutex<bool> = &self._canPush.0;
        let condVar_push = &self._canPush.1;
        let mut mutex_push = lock_push.lock().unwrap();
        while !*mutex_push {
            mutex_push = condVar_push.wait(mutex_push).unwrap();
        }

        if self.size - self._queue.len() == 1 {
            *mutex_push = false;
            condVar_push.notify_all();
        }
        self._queue.push(json_message);
        }
        let (lock_pull, condVar_pull) = &*self._canPull;
        let mut mutex_pull = lock_pull.lock().unwrap();
        *mutex_pull = true;
        condVar_pull.notify_all();
    }

    fn pull(&self) -> Vec<u8> {
        let (lock_pull, condVar_pull) = &*self._canPull;
        let mut mutex_pull = lock_pull.lock().unwrap();
        while !*mutex_pull {
            mutex_pull = condVar_pull.wait(mutex_pull).unwrap();
        }
        if self._queue.len() == 1 {
            *mutex_pull = false;
            condVar_pull.notify_all();
        }
        drop(mutex_pull);
        let message = self._queue.pop().unwrap();
        let (lock_push, condVar_push) = &*self._canPush;
        let mut mutex_push = lock_push.lock().unwrap();
            *mutex_push = true;
            condVar_push.notify_all();
        message

    }
}

impl Queue {
    pub fn get_size(&self) -> usize {
        self._queue.len()
    }
}

pub trait Resizable {
    fn set_size(&mut self, c_size: usize);
}

pub trait QueueOperations<T> {
    fn push(&self, json_message: T);
    fn pull(&self) -> T;
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

//impl QueueOperations<String> for CircularQueue {
//
//    fn push(&mut self, json_message: String) {
//        if self.c_queue.capacity() == self.c_size {
//            self.save_in_file(json_message);
//        } else {
//            self.c_queue.push(json_message);
//        }
//    }
//
//    fn pull(&mut self) -> String {
//        let mut message = self.c_queue.pop().unwrap_or_default();
//        self.get_file_message();
//        return String::from(message.trim_end_matches("\n"));
//    }
//}
//
//impl Resizable for CircularQueue {
//
//    fn set_size(&mut self, c_size: usize) {
//        self.c_size = min(c_size, 100000);
//    }
//}
//
//impl CircularQueue {
//    fn save_in_file(&mut self, json_message: String) {
//        let mut file = BufWriter::new(self.write_pointer.borrow_mut());
//        file.write_all((json_message + "\n").as_bytes())
//            .expect("Unable to write into the log file");
//    }
//
//    fn get_file_message(&mut self) {
//        //Does not remove lines, will have to look into it.
//        let reader = self.read_pointer.borrow_mut();
//        let mut line_string: String = String::from("");
//        reader.read_line(line_string.borrow_mut()).unwrap();
//        if !line_string.is_empty() {
//            self.c_queue.push(line_string);
//        }
//    }
//}
//
//#[test]
//fn default_test() {
//    let mut _cqueue: CircularQueue = Default::default();
//    assert_eq!(_cqueue.c_size, 100000);
//    _cqueue.set_size(100);
//    assert_eq!(_cqueue.c_size, 100);
//
//    _cqueue.push(String::from("Hello"));
//    assert_eq!(_cqueue.pull(), "Hello");
//}
//
//#[test]
//fn operation_test() {
//    let mut _cqueue = CircularQueue {
//        write_pointer: File::create("tortoise_test.log").unwrap(),
//        read_pointer: BufReader::new(File::open("tortoise_test.log").unwrap()),
//        ..Default::default()
//    };
//    _cqueue.set_size(1);
//    _cqueue.push("1".to_string());
//    _cqueue.push("2".to_string());
//    _cqueue.push("3".to_string());
//    _cqueue.push("4".to_string());
//    _cqueue.push("5".to_string());
//    assert_eq!("1", _cqueue.pull());
//    _cqueue.push("6".to_string());
//    assert_eq!("2", _cqueue.pull());
//    assert_eq!("3", _cqueue.pull());
//    assert_eq!("4", _cqueue.pull());
//    assert_eq!("5", _cqueue.pull());
//    assert_eq!("6", _cqueue.pull());
//}
