use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    
    //------------------------------------------
    // Using Threads to Run Code Simultaneously
    //------------------------------------------

    {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        // Rust infers how to capture v, and because println! only needs a reference to v, 
        // the closure tries to borrow v

        // By adding the move keyword before the closure, we force the closure to take 
        // ownership of the values it’s using rather than allowing Rust to infer that it 
        // should borrow the values

        let handle = thread::spawn(move || {
            for i in &v {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1));
            }
        });

        // ❌ By telling Rust to move ownership of v to the spawned thread, we’re guaranteeing 
        //    to Rust that the main thread won’t use v anymore
        // drop(v);

        for i in 1..5 {
            println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }

        // Calling join on the handle blocks the thread currently running until the thread 
        // represented by the handle terminates

        handle.join().unwrap();
    }

    //----------------------------------------------------
    // Transfer Data Between Threads with Message Passing
    //----------------------------------------------------

    {
        // `mpsc` stands for multiple producer, single consumer
        let (tx, rx) = mpsc::channel();

        let tx1 = tx.clone();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {

                // The send method returns a Result<T, E> type, so if the receiver has
                // already been dropped and there’s nowhere to send a value, the send 
                // operation will return an error

                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));

                // ❌ The send function takes ownership of its parameter, and when the 
                //     value is moved the receiver takes ownership of it

                // println!("val is {val}");
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        // `recv`, short for receive, will block the main thread’s execution and wait
        // until a value is sent down the channel. Once a value is sent, recv will return
        // it in a Result<T, E>. When the transmitter closes, recv will return an error
        // to signal that no more values will be coming

        // The try_recv method doesn’t block, but will instead return a Result<T, E> 
        // immediately: an Ok value holding a message if one is available and an Err value 
        // if there aren’t any messages this time

        // let received = rx.recv().unwrap();

        for received in rx {
            println!("Got: {received}");
        }
    }

    //--------------------------
    // Shared-State Concurrency
    //--------------------------

    // Controlling Access with Mutexes

    // 1. You must attempt to acquire the lock before using the data.
    // 2. When you’re done with the data that the mutex guards, you must unlock the 
    //    data so that other threads can acquire the lock.

    // Mutex<T> comes with the risk of creating deadlocks. These occur when an operation 
    // needs to lock two resources and two threads have each acquired one of the locks, 
    // causing them to wait for each other forever

    {
        // Arc<T> is a type like Rc<T> that is safe to use in concurrent situations

        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);

            // To access the data inside the mutex, we use the lock method to acquire 
            // the lock. This call will block the current thread so that it can’t do 
            // any work until it’s our turn to have the lock

            // The exact type of num is std::sync::MutexGuard<'_, i32>

            // The MutexGuard type implements Deref to point at our inner data; the 
            // type also has a Drop implementation that releases the lock automatically 
            // when a MutexGuard goes out of scope, which happens at the end of the 
            // inner scope

            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }

    //-------------------------------------------
    // Extensible Concurrency with Send and Sync
    //-------------------------------------------

    // The ownership of values of the type implementing `Send` can be transferred between threads

    // it is safe for the type implementing `Sync` to be referenced from multiple threads
    
}