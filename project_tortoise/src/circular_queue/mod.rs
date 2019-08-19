
extern crate queues;

use queues::*;
use std::fs::{OpenOptions,File};
use std::io::{BufReader,BufWriter};
use std::string::String;



    struct CircularQueue {
        c_queue : Queue<String>,
        c_size : i32,
        file_pointer : OpenOptions
    }

    trait QueueOperations {
        fn set_size(self : &mut Self, c_size: i32);
        fn push( self : &mut Self, json_message : String);
        fn pull( self : &mut Self);
        fn save_in_file(self : &mut Self,json_message: String);
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

        fn push(self : &mut Self, json_message : String) {

            if self.c_queue.size() == self.c_size {
                saveInFile(self.c_queue.peek());
                self.c_queue.remove();
                self.c_queue.add(json_message);
            }
            else {
                self.c_queue.add(json_message);
            }
        }

        fn pull(self : &mut Self){
            //Does not remove lines, will have to look into it.
            let file = BufReader::new(self.file_pointer);
            for line in file.lines() {
                if self.c_queue.size() < self.c_size {
                    self.c_queue.add(String::from_utf8(line));
                }
                else {
                    break;
                }
            }
        }

        fn save_in_file(self : &mut Self, json_message : String){
            let  file = Buf::new(self.file_pointer);
            file.write_all(json_message.trim().as_bytes()).expect("Unable to write into the log file");

        }
    }
