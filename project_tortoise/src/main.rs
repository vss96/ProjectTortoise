mod circular_queue;

use crate::circular_queue::{CircularQueue, QueueOperations, Queue};
use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    let _cqueue: Arc<Queue> = Arc::new(Queue::default());
    let queue_clone = _cqueue.clone();
    thread::spawn(move || {
        sleep(Duration::new(1, 0));
        for i in 0..50000 {
            sleep(Duration::from_millis(1));
            let message1 = String::from_utf8(queue_clone.pull());
            println!("pulled_message: {}", message1.unwrap());
        }
    });
    for i in 0..150000 {
        let mut string = String::from("Random_String_");
        string.push_str(i.to_string().as_str());
        _cqueue.push(string.into_bytes());
    }
    println!("Finished inserting all elements: {}", _cqueue.get_size().to_string());
    let msg = String::from_utf8(_cqueue.pull());
}
