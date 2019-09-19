use std::sync::Arc;
use std::{fs, thread};

use crate::circular_queue::{Queue, QueueOperations};
use std::borrow::Borrow;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

mod circular_queue;

fn write_to_connection(mut stream: TcpStream, queue: Arc<Queue>) {
    while true {
        let msgBytes = queue.pull();
        match msgBytes {
            Ok(msg) => {
                stream.write_all(msg.borrow());
                stream.write_all("\n".as_bytes());
            },
            Err(E) => {

            }
        }
    }
}

fn spawn_push_thread(port: String, queue: Arc<Queue>) {
    let addr = "0.0.0.0:".to_owned() + port.as_str();
    let listener = TcpListener::bind(&addr).unwrap();
    println!("Server started at {}", &addr);
    for connection in listener.incoming() {
        match connection {
            Ok(mut stream) => {
                println!("Server is geting pinged");
                let server_queue = queue.clone();
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

fn main() {
    let _cqueue: Arc<Queue> = Arc::new(Queue::default());
    let queue_clone = _cqueue.clone();

    let paths = fs::read_dir("/data/finalDataSet4Sept2019/").unwrap();
    let mut a = 1;
    thread::spawn(move || {
        for path in paths {
            let fname = String::from(path.unwrap().path().to_str().unwrap());
            let json = fs::read_to_string(fname).unwrap_or_default().into_bytes();
            queue_clone.push(json);
        }
    });
    let clonedQueue = _cqueue.clone();
    thread::spawn(move || {
        spawn_push_thread(String::from("6008"), clonedQueue);
    });
    spawn_push_thread(String::from("6007"), _cqueue.clone());
}
