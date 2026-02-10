// THREADS
// They allow us to make a program to run tasks concurrently, improving performance and responsiveness.

use std::thread; // We spawn more threads with it

fn main() {
    thread::spawn(f);
    thread::spawn(f);

    println!("Hello from the main thread.");
}

fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}


// ============================================================================================
// As the main thread may finish before the spawned threads, we prefer to use the join handle

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("Hello from the main thread.");

    t1.join().unwrap(); // Join returns a Result: "Err" if the thread panicked / "Ok" otherwise
    t2.join().unwrap();
}

// ============================================================================================
// Example

use std::thread;

fn main() {
    let t = thread::spawn(|| panic!("Oops - panicked again"));
    t.join().expect("Joining thread...");
}

// ============================================================================================
// Closures and ownership
// Syntax: Closure are defined using vertical pipes | |

use std::thread;
fn main() {
    let numbers = vec![1, 2, 3];
    
    thread::spawn(move || {
        for n in &numbers {
            println!("{n}");
        }
    }).join().unwrap();
}

// ============================================================================================
// Threads: Scoped /1

use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];
    thread::scope(|s| { // s is he scope
        s.spawn(|| {
            println!("length: {}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
    }); // This way, all spawned threads are joined
}

// ============================================================================================
// Threads: Scoped /2
// Usual reference rules apply: one mutable, no immutable OR many immutable

use std::thread;
fn main() {

    let mut numbers = vec![1, 2, 3];

    thread::scope(|s| {
        s.spawn(|| {
            numbers.push(1); // numbers captured by &mut
        });
        s.spawn(|| {
            numbers.push(2); // Error!
        });
    });

}

// ============================================================================================
// Thread Builder
// std::thread::spawn is a shorthand for std::thread::Builder::new().spawn().unwrap
// Using thread::Builder allows us to change name, stack size, etc.

let builder = thread::Builder::new()
    .name("worker-1".into())
    .stack_size(4 * 1024 * 1024); // 4 MB


let handler = builder.spawn(|| {
    // Use like any old thread spawn
}).unwrap(); // Unwrapped as spawn could fail (resource limits, out of mem)

// We can also use more robust code to handle errors

let handler = builder.spawn(|| {
    // Work
}).expect("Failed to spawn worker thread"); // In more robust code

// OR

match builder.spawn(|| {}) {
    Ok(h) => h,
    Err(e) => {
        log::error!("Thread spawn failed: {e}");
        return;
    }
}
 
// Threads Summary:
/*
- Spawn threads with a closure
- Join threads to make sure they complete
- Use move keyword to take ownership of data
- Use scoped threads to allow parent thread's non-static data to be referenced
*/