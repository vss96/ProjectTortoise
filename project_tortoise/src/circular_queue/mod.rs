
extern crate queues;

use queues::*;
use std::fs::{OpenOptions,File};
use std::io::{BufReader,BufferWriter};
use std::string::String;



    struct CircularQueue {
        c_queue : Queue<String>,
        c_size : i32,
        file_pointer : OpenOptions
    }

    trait QueueOperations {
        fn set_size(self : &mut Self, c_size: i32);
        fn push( self : &mut Self, message_id : String);
        fn pull( self : &mut Self);
        fn save_in_file(self : &mut Self,json_message: String);
        fn read_file( self : &mut Self) -> String;
    }

    impl Default for CircularQueue{
        fn default() -> CircularQueue{
            CircularQueue{
                c_queue : queue![],
                c_size : 100000,
                file_pointer : File::create("tortoise.log")
            }
        }
    }

    impl QueueOperations for CircularQueue {
        fn set_size(self: &mut Self, c_size: i32){
            self.c_size = c_size;
        }

        fn push(self : &mut Self, message_id : String) {

            if self.c_queue.size() == self.c_size {
                saveInFile(self.c_queue.peek());
                self.c_queue.remove();
                self.c_queue.add(message_id);
            }
            else {
                self.c_queue.add(message_id);
            }
        }

        fn pull(self : &mut Self){

            let mut message : String;
            while self.c_queue.size() < self.c_size {
                message = read_file(self);
                self.c_queue.add(message)
            }

        }

        fn save_in_file(self : &mut Self, json_message : String){
            let  file = BufferWriter::new(self.file_pointer);
            file.write_all(json_message.as_bytes()).expect("Unable to write into the log file");

        }

        fn read_file(self : &mut Self) -> String {
            let file = BufferReader::new(self.file_pointer);

        }
    }
