use std::{fs, thread};
use std::borrow::Borrow;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

use threadpool::ThreadPool;

use crate::circular_queue::{Queue, QueueOperations};

mod circular_queue;

fn write_to_connection(mut stream: TcpStream, queue: Arc<Queue>) {
    loop {
        let msg_bytes = queue.pull();
        match msg_bytes {
            Ok(msg) => {
                stream.write_all(msg.borrow()).unwrap_or_default();
                stream.write_all("\n".as_bytes()).unwrap_or_default();
            }
            Err(e) => {
                if queue.is_queue_ended() {
                    break;
                }
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
                println!("Server is geting pinged on {}", &port);
                let server_queue = queue.clone();
                thread::spawn(move || {
                    write_to_connection(stream, server_queue);
                });
            }
            Err(e) => {
                println!("Socket failed");
            }
        }
    }
}

fn main() {
    let _cqueue: Arc<Queue> = Arc::new(Queue::default());
    let queue_clone = _cqueue.clone();
    let paths = fs::read_dir("/data/finalDataSet4Sept2019").unwrap();

    let spawn_queue = _cqueue.clone();
    let join_handle_server = thread::spawn(move || {
        spawn_push_thread(String::from("6881"), spawn_queue.clone());
    });

    let mut file_count = 0;
    let n_workers = 2;
    let join_handle_file_reader_thread = thread::spawn(move || {
        let pool = ThreadPool::new(n_workers);
        for path in paths {
            let queue_clone_for_worker = queue_clone.clone();
            pool.execute(move || {
                let fpath = &path;
                let fname = String::from(fpath.as_ref().unwrap().path().file_stem().unwrap().to_str().unwrap());
                let pname = String::from(path.unwrap().path().to_str().unwrap());
                let mut json = fs::read(pname).unwrap_or_default();
                let mut appended_file_name = (":".to_owned() + &fname).into_bytes();
                let queue_msg = json.append(&mut appended_file_name);
                queue_clone_for_worker.push(json);
                file_count += 1;
            });
        }
        pool.join();
        queue_clone.end_queue();
    });
    join_handle_file_reader_thread.join().unwrap();
    join_handle_server.join().unwrap();
}
