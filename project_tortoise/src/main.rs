use std::sync::Arc;
use std::{fs, thread};

use crate::circular_queue::{Queue, QueueOperations};
use std::borrow::Borrow;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

mod circular_queue;

fn write_to_connection(mut stream: TcpStream, queue: Arc<Queue>) {
    while true {
        stream.write_all(queue.pull().borrow());
        stream.write_all("\n".as_bytes());
    }
}

fn main() {
    let _cqueue: Arc<Queue> = Arc::new(Queue::default());
    let queue_clone = _cqueue.clone();

    let paths = fs::read_dir("/Users/vikass/Documents/sampleData").unwrap();
    let mut a = 1;
    thread::spawn(move || {
        for path in paths {
            let fname = String::from(path.unwrap().path().to_str().unwrap());
            let json = fs::read_to_string(fname).unwrap_or_default().into_bytes();
            queue_clone.push(json);
        }
    });

    let addr = "127.0.0.1:6007";
    let listener = TcpListener::bind(&addr).unwrap();
    println!("Server started");
    for connection in listener.incoming() {
        match connection {
            Ok(mut stream) => {
                println!("Server is geting pinged");
                let server_queue = _cqueue.clone();
                thread::spawn(move || {
                    write_to_connection(stream, server_queue);
                });
            }
            Err(E) => {
                println!("Socket failed");
            }
        }
    }
}
