use std::sync::Arc;
use std::{fs, thread};

use crate::circular_queue::{Queue, QueueOperations};
use std::borrow::Borrow;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpListener;
use tokio::prelude::{Future, Stream};
use tokio::timer::Interval;

mod circular_queue;

fn main() {
    let _cqueue: Arc<Queue> = Arc::new(Queue::default());
    let queue_clone = _cqueue.clone();
    //    thread::spawn(move || {
    //        sleep(Duration::new(1, 0));
    //        for i in 0..50000 {
    //            sleep(Duration::from_millis(1));
    //            let message1 = String::from_utf8(queue_clone.pull());
    //            println!("pulled_message: {}", message1.unwrap());
    //        }
    //    });
    //    for i in 0..150000 {
    //        let mut string = String::from("Random_String_");
    //        string.push_str(i.to_string().as_str());
    //        _cqueue.push(string.into_bytes());
    //    }
    //    println!(
    //        "Finished inserting all elements: {}",
    //        _cqueue.get_size().to_string()
    //    );
    //    let msg = String::from_utf8(_cqueue.pull());

    let paths = fs::read_dir("sampleData").unwrap();
    let mut a = 1;
    thread::spawn(move || {
        for path in paths {
            let fname = String::from(path.unwrap().path().to_str().unwrap());
            let json = fs::read_to_string(fname).unwrap_or_default().into_bytes();
            queue_clone.push(json);
        }
    });

    let addr = "127.0.0.1:8080".parse().expect("Unable to parse address");
    let listener = TcpListener::bind(&addr).unwrap();

    let done = listener
        .incoming()
        .for_each(move |socket| {
            let server_queue = _cqueue.clone();
            let (reader, mut writer) = socket.split();
            let sender = Interval::new_interval(std::time::Duration::from_millis(1))
                .for_each(move |_| {
                    writer
                        .poll_write(server_queue.pull().borrow())
                        .map_err(|_| {
                            tokio::timer::Error::shutdown();
                        })
                        .unwrap();
                    return Ok(());
                })
                .map_err(|e| println!("{}", e));
            ;
            tokio::spawn(sender);
            return Ok(());
        })
        .map_err(|e| println!("Future_error {}", e));

    tokio::run(done);
}
