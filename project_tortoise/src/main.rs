use std::sync::Arc;
use std::{fs, thread};

use crate::circular_queue::{Queue, QueueOperations};
use std::borrow::Borrow;
use std::io::Write;
use std::net::TcpListener;

mod circular_queue;

fn main() {
    let _cqueue: Arc<Queue> = Arc::new(Queue::default());
    let queue_clone = _cqueue.clone();

    let paths = fs::read_dir("../../Documents/sampleData").unwrap();
    let mut a = 1;
    thread::spawn(move || {
        for path in paths {
            let fname = String::from(path.unwrap().path().to_str().unwrap());
            let json = fs::read_to_string(fname).unwrap_or_default().into_bytes();
            queue_clone.push(json);
        }
    });

    let addr = "0.0.0.0:6006";
    let listener = TcpListener::bind(&addr).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut wx) => {
                let server_queue = _cqueue.clone();
                while server_queue.get_size() == 0 {
                    wx.write_all(server_queue.pull().borrow());
                    wx.write_all("\n".as_bytes());
                }
            }
            Err(E) => {
                println!("Socket failed");
            }
        }
    }
}
