extern crate queues;

use std::borrow::{Borrow, BorrowMut};
use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::string::String;
use std::sync::{Arc, Condvar, Mutex};

use crossbeam_queue::ArrayQueue;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Queue {
    name: String,
    size: usize,
    end_marker: AtomicBool,
    _queue: ArrayQueue<Vec<u8>>,
}

impl Default for Queue {
    fn default() -> Self {
        Queue {
            name: String::from("Default"),
            size: 500000,
            end_marker: AtomicBool::new(false),
            _queue: ArrayQueue::new(500000),
        }
    }
}

impl QueueOperations<Vec<u8>> for Queue {
    fn push(&self, json_message: Vec<u8>) {
        while self._queue.len() == self.size {} //wait if queue is full
        self._queue.push(json_message);
    }

    fn pull(&self) -> Result<Vec<u8>, crossbeam_queue::PopError> {
        let message = self._queue.pop();
        message
    }
}

impl Queue {
    pub fn end_queue(&self) {
        self.end_marker.store(true, Ordering::Relaxed);
    }

    pub fn is_queue_ended(&self) -> bool {
        self.end_marker.load(Ordering::Relaxed)
    }
}

pub trait QueueOperations<T> {
    fn push(&self, json_message: T);
    fn pull(&self) -> Result<T, crossbeam_queue::PopError>;
}

#[test]
fn default_test() {
    let mut _cqueue: Queue = Default::default();
    assert_eq!(_cqueue.size, 500000);
    _cqueue.push(String::from("Hello").as_bytes().to_vec());
    assert_eq!(String::from_utf8(_cqueue.pull().unwrap()).unwrap(), "Hello");
}
