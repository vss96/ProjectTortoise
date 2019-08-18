
extern crate queues;

use queues::*;


    struct circular_queue{
        cQueue : Queue<String>,
        cSize : i32
    }

    impl circular_queue {
        fn setSize(self: &mut Self, cSize : i32){
            self.cSize = cSize;
        }

        fn push(self : &mut Self, messageId : String) {
            if self.cQueue.size() == self.cSize {
                saveInFile(self.cQueue.peek());
                self.cQueue.remove();
                self.cQueue.add(messageId);
            }
            else {
                self.cQueue.add(messageId);
            }
        }

        fn saveInFile(messageId : String){
            //find file by messageId and store it to disk
        }
    }
