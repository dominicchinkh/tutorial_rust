use std::thread;
use std::time::Duration;
use trpl::{Either, Html, StreamExt};

/*
 *  When Rust sees a function marked with async, it compiles it into a non-async 
 *  function whose body is an async block. When Rust sees a block marked with the 
 *  async keyword, it compiles it into a unique, anonymous data type that implements
 *  the Future trait
 *
 *  This function is is roughly equivalent to a non-async function defined like this:
 *
 *    fn page_title(url: &str) -> impl Future<Output = Option<String>> {
 *        async move {
 *            let text = trpl::get(url).await.text().await;
 *            Html::parse(&text)
 *                .select_first("title")
 *                .map(|title| title.inner_html())
 *        }
 *    }
 *
 */
async fn page_title(url: &str) -> (&str, Option<String>) {
    
    // `futures` in Rust are lazy: they don’t do anything until you ask them to with
    // the await keyword

    let response = trpl::get(url).await;
    let response_text = response.text().await;

    let title = Html::parse(&response_text)
                .select_first("title")
                .map(|title| title.inner_html());

    (url, title)
}

// Execute with: cargo run -- "https://www.rust-lang.org" "https://www.google.com"
fn main() {

    //------------------------------
    // Futures and the Async Syntax
    //------------------------------

    let args: Vec<String> = std::env::args().collect();

    {
        /*
        *  1. Each await point—that is, every place where the code uses the await 
        *     keyword—represents a place where control is handed back to the runtime
        *
        *  2. To make that work, Rust needs to keep track of the state involved in the 
        *     async block so that the runtime could kick off some other work and then 
        *     come back when it’s ready to try advancing the first one again
        *
        *  3. runtime executes this state machine
        */

        /*
        *  `block_on` function takes a future as an argument and blocks the current 
        *  thread until this future runs to completion. Behind the scenes, calling 
        *  `block_on` sets up a runtime using the tokio crate that’s used to run the 
        *  future passed in. Once the future completes, `block_on` returns whatever 
        *  value the future produced
        *
        */
        trpl::block_on(async {
            let url = &args[1];

            let (url, title) = page_title(url).await;

            match title {
                Some(title) => println!("The title for {url} was {title}"),
                None => println!("{url} had no title"),
            }
        })
    }

    {
        trpl::block_on(async {
            let title_fut_1 = page_title(&args[1]);
            let title_fut_2 = page_title(&args[2]);

            // These don’t do anything yet, because futures are lazy and we haven’t yet
            // awaited them

            let (url, maybe_title) =

                // The select function returns Left with that future’s output if the first 
                // argument wins, and Right with the second future argument’s output if 
                // that one wins

                match trpl::select(title_fut_1, title_fut_2).await {
                    Either::Left(left) => left,
                    Either::Right(right) => right,
                };

            println!("{url} returned first");

            match maybe_title {
                Some(title) => println!("Its page title was: '{title}'"),
                None => println!("It had no title."),
            }
        })
    }

    //---------------------------------
    // Applying Concurrency with Async
    //---------------------------------

    {
        trpl::block_on(async {
            
            let handle = trpl::spawn_task(async {
                for i in 1..10 {
                    println!("hi number {i} from the first task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            });

            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }

            // If you want it to run all the way to the task’s completion, you will need
            // to use a join handle to wait for the first task to complete
            
            handle.await.unwrap();
        });
    }

    {
        trpl::block_on(async {

            // We can put each loop in an async block and have the runtime run them both 
            // to completion using the trpl::join function

            let fut1 = async {
                for i in 1..10 {
                    println!("hi number {i} from the first task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            let fut2 = async {
                for i in 1..5 {
                    println!("hi number {i} from the second task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            // You’ll see the exact same order every time. `trpl::join` function is fair, 
            // meaning it checks each future equally often, alternating between them, 
            // and never lets one race ahead if the other is ready

            trpl::join(fut1, fut2).await;
        });
    }

    //------------------------------------------------------
    // Sending Data Between Two Tasks Using Message Passing

    {
        trpl::block_on(async {
            let (tx, mut rx) = trpl::channel();

            let tx1 = tx.clone();
            let tx1_fut = async move {
                let vals = vec![
                    String::from("more"),
                    String::from("messages"),
                    String::from("for"),
                    String::from("you"),
                ];

                for val in vals {
                    tx1.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(1500)).await;
                }
            };

            // We put the tx and rx operations in their own async blocks. Code within 
            // one async block executes linearly

            // if we could move tx into that async block, it would be dropped once that 
            // block ends

            let tx_fut = async move {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("future"),
                ];

                // We don’t await the send call, because it doesn’t block

                for val in vals {
                    tx.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            // The trpl::Receiver::recv method does not block. It hands control back to
            // the runtime until either a message is received or the send side of the 
            // channel closes

            let rx_fut = async {

                // If the result of calling rx.recv().await is Some(message), we get access
                // to the message and we can use it in the loop body. If the result is None, 
                // the loop ends. Every time the loop completes, it hits the await point 
                // again, so the runtime pauses it again until another message arrives

                // Awaiting rx.recv will return None only once the other end of the channel 
                // is closed. The channel will close only when the sender side, tx, is dropped

                while let Some(value) = rx.recv().await {
                    println!("received '{value}'");
                }
            };

            // The join! macro awaits an arbitrary number of futures where we know the 
            // number of futures at compile time

            trpl::join!(tx1_fut, tx_fut, rx_fut);
        });
    }

    //---------------------------------
    // Yielding Control to the Runtime

    {
        trpl::block_on(async {
            let one_ms = Duration::from_millis(1);

            let a = async {
                println!("'a' started.");
                slow("a", 30);

                // Hand back control to the runtime
                trpl::yield_now().await;

                slow("a", 10);
                trpl::yield_now().await;
                slow("a", 20);
                trpl::yield_now().await;
                println!("'a' finished.");
            };

            let b = async {
                println!("'b' started.");
                slow("b", 75);
                trpl::yield_now().await;
                slow("b", 10);
                trpl::yield_now().await;
                slow("b", 15);
                trpl::yield_now().await;
                slow("b", 350);
                trpl::yield_now().await;
                println!("'b' finished.");
            };

            trpl::join(a, b).await;
        });
    }

    //-------------------------------------
    // Building Our Own Async Abstractions

    {
        trpl::block_on(async {

            let slow = async {
                trpl::sleep(Duration::from_secs(5)).await;
                "Finally finished"
            };

            async fn timeout<F: Future>(
                future_to_try: F,
                max_time: Duration,
            ) -> Result<F::Output, Duration> {

                // Note: The implementation of trpl::select is not fair: it always  
                // polls arguments in the order in which they are passed. Thus, we  
                // pass future_to_try to select first so it gets a chance to complete 
                // even if max_time is a very short duration

                match trpl::select(future_to_try, trpl::sleep(max_time)).await {
                    Either::Left(output) => Ok(output),
                    Either::Right(_) => Err(max_time),
                }
            }

            match timeout(slow, Duration::from_secs(2)).await {
                Ok(message) => println!("Succeeded with '{message}'"),
                Err(duration) => {
                    println!("Failed after {} seconds", duration.as_secs())
                }
            }
        });
    }

    //------------------------------
    // Streams: Futures in Sequence
    //------------------------------

    {
        trpl::block_on(async {
            let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

            let iter = values.iter().map(|n| n * 2);
            let mut stream = trpl::stream_from_iter(iter);

            while let Some(value) = stream.next().await {
                println!("The value was: {value}");
            }
        });
    }

    //---------------------------------------
    // A Closer Look at the Traits for Async
    //---------------------------------------

    {
        use std::pin::Pin;
        use std::task::{Context, Poll};

        pub trait Future {
            type Output;

            /*
             *  The Poll type:
             *    The Pending variant indicates that the future still has work to do, so
             *    the caller will need to check again later. The Ready variant indicates 
             *    that the Future has finished its work and the T value is available
             *
             *    pub enum Poll<T> {
             *      Ready(T),
             *      Pending,
             *    }
             */

            /*
             *  Pin
             *    Pin is a wrapper type (Pin<P>) that wraps a pointer type P (such as &mut T, 
             *    Box<T>, Rc<T>). It acts as a contract: "The value at the end of this pointer 
             *    will stay at its current memory address until it is dropped."
             *
             *    Because the underlying value cannot move:
             *      + You cannot take a mutable reference (&mut T) to a pinned type
             *      + Self-referential pointers inside T remain valid for the entire lifetime 
             *        of T
             *
             *  Unpin
             *    Unpin is an auto-trait (like Send or Sync). It is implemented automatically by
             *    the compiler for almost all standard Rust types (e.g., i32, String, Vec, custom
             *    structs containing Unpin fields)
             *
             *    T: Unpin means: "This type is completely safe to move even if wrapped in a Pin."
             *    T: !Unpin means: "This type is NOT safe to move once pinned. Keep it in place!"
             */

            /*
             *  Why are Pin and Unpin Essential for async/await?
             *
             *    1. When you write an async fn or async block, the Rust compiler lowers it into 
             *       an anonymous struct that acts as a state machine. Each .await point represents
             *       a yield state where the function can pause and resume execution later
             *
             *    2. If an async function creates a local variable and borrows it across an .await 
             *       point, both the variable and its reference must be stored inside the compiler 
             *       generated state machine struct so they survive across yield points
             *
             *       Because one field in the struct points directly to another field in the same 
             *       struct, the struct becomes self-referential. Moving that struct to a new memory
             *       address between .await points would invalidate the reference, pointing to old 
             *       memory (a dangling reference)
             *
             *    3. Therefore, Rust requires Future::poll to receive self as a pinned reference
             */

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
        }
    }

    //------------------------------------------------------
    // Putting It All Together: Futures, Tasks, and Threads
    //------------------------------------------------------

    {
        let (tx, mut rx) = trpl::channel();

        thread::spawn(move || {
            for i in 1..11 {
                tx.send(i).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        trpl::block_on(async {
            while let Some(message) = rx.recv().await {
                println!("{message}");
            }
        });
    }
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}
