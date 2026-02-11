// Shared Ownership
// Static data has its lifetime tied to the length of the program

static NUMBERS: [i32; 3] = [1, 2, 3]

// By convention, we use SCREAMING_SNAKE_CASE to declare them 
// As the thread can't live more than the main function, 'thread::spawn' doesn't run, requiring
// closure to be ''static'
let mut numbers: Box<[usize]> = Vec::from_iter(0..=1000).into_boxed_slice();
let numbers: &mut [usize] = &mut numbers[..];

let t = thread::spawn(|| { // Error on numbers lifetime
    let len = numbers.len();
    let sum = numbers.iter().sum::<usize>();
    sum / len
});

let average = t.join().unwrap();

// In order to make our data static, we can leak the memory
let numbers: Box<[usize]> = Vec::from_iter(0..=1000).into_boxed_slice();
let numbers: &'static mut [usize] = Box::leak(numbers);

let t = thread::spawn(|| {
    let len = numbers.len();
    let sum = numbers.iter().sum::<usize>();
    sum / len
});

let average = t.join().unwrap();

// Reference Counting
// Allows us to have multiple owners of the same reference
std::rc::Rc //for single-threaded use cases -> Rc = Reference Counted

std::sync::Arc // for multi-threaded  -> Arc = Atomic Reference Counted

// Create multiple owner by cloning the 'Rc' or 'Arc'
use std::sync::Arc;

let a = Arc::new([1, 2, 3]); //1
let b = a.clone(); // 2

thread::spawn(move || dbg!(a)); //3
thread::spawn(move || dbg!(b)); //3

// Smart naming
let a = Arc::new([1, 2, 3]);

thread::spawn({ // Scope starts
    let a = a.clone(); // Shadows outer 'a'
    move || {
        dbg!(a);
    }
}); // Scope ends, inner 'a' goes out scope

dbg!(a); // Original 'a'

// Unsafe an UB (Undefined Behavior)
let a = [10, 20, 30];
let index = 10; // Wrong 

let x = unsafe {a.get_unchecked(index)};
println!("{x}"); // It may print garbage, crash or appear to work

// Interior Mutability 
// Mutates data through an immutable reference by checking borrowing rules at runtime, using types like `Cell`, `RefCell`, etc.
// Mutability Container types:

/* Cell<T> (simple interior mutability)
- .get() and .set() methods for modifying
- single thread only
                                                                                                                                */
//Ex:

use std::cell::Cell;

let x = Cell::new(1);
x.set(2);
println!("{}", x.get());

/* RefCell<T> (runtime borrow checking)
- can borrow contents with .borrow() and .borrow_mut()
- Reference rules checked at runtime
- panics if it can't get the borrow
- single thread only
                                                        */
//Ex:

use std::cell::RefCell;

let x = RefCell::new(1);
*x.borrow_mut() += 1;
println!("{}", x.borrow());

/* RwLock<T> (multiple readers OR one writer)
- concurrent version of RefCell
- blocks/sleeps instead of calling panic if it can't get a borrow
                                                                    */
// Ex:

use std::sync::RwLock;

fn main() {
    let value = RwLock::new(10);

    let r = value.read().unwrap();
    println!("Read: {}", *r);
    drop(r);

    let mut w = value.write().unwrap();
    *w += 5;

    println!("Final: {}", *value.read().unwrap());
}

/* Mutex<T> (exclusive access only)
- simpler than RwLock
- only allows exclusive borrows
                                    */
// Ex:

use std::sync::Mutex;

fn main() {
    let counter = Mutex::new(0);

    *counter.lock().unwrap() += 1;
    *counter.lock().unwrap() += 1;

    println!("{}", *counter.lock().unwrap());
}

/* Atomics (AtomicUsize, etc.)
- concurrent version of a Cell
- platform dependent - only supports certain T (e.g. ptr, u32, etc.)
                                                                    */
// Ex:

use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    let counter = AtomicUsize::new(0);

    counter.fetch_add(1, Ordering::SeqCst);
    counter.fetch_add(1, Ordering::SeqCst);

    println!("Counter: {}", counter.load(Ordering::SeqCst));
}

/* UnsafeCell
- get() method gives access to the data's raw ptr
- building block underpinning all the other types
                                                    */
// Ex:
pub struct Cell<T: ?Sized> {
    value: UnsafeCell<T>,
}
impl<T: Copy> Cell<T> {
    pub fn get(&self) -> T {
        // SAFETY: This can cause data races if called from a separate thread
        // but `Cell` is `Sync` so this won't happen.
        unsafe { *self.value.get() }
    }
}

/* Thread Safety: Send and Sync
Traits to inform the compiler of the thread safety behavior of a given type:
- Send -> T can be sent to another thread;
- Sync -> T can be shared with another thread, i.e. &T can be sent.
These traits are usually auto-implemented.

Raw pointer are not sync or send.
The traits can be opted-in to do with an unsafe impl block, for example:
                                                                            */
// Ex:
struct X {
    p: *mut i32,
}

unsafe impl Send for X {}
unsafe impl Sync for X {}
/* 
unsafe indicates that the compiler cannot check the safety.

If a type is not send, you can't move it onto another thread:
                                                            */
// Ex:
fn main() {
    let a = Rc::new(123);
    thread::spawn(move || { // Error
        dbg!(a);
    });
}

/*We can opt out of the traits by adding a PhantomData type 
containing something which is, for example, not Sync.
                                                            */
use std::marker::PhantomData;

struct X {
    handle: i32,
    _not_sync: PhantomData<Cell<()>>, // Cell is not Sync, so X is not Sync
}








