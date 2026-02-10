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
/* 
Cell<T> (simple interior mutability)
- .get() and .set() methods for modifying
- single thread only
*/
//Ex:

use std::cell::Cell;

let x = Cell::new(1);
x.set(2);
println!("{}", x.get());

/* 
RefCell<T> (runtime borrow checking)
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





