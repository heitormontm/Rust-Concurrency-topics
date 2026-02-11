
/* 
    Thread Safety: Send and Sync
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
use std::marker::PhantomData; // PhantomData has zero runtime size, it exists only for the type system

struct X {
    handle: i32,
    _not_sync: PhantomData<Cell<()>>, // Cell is not Sync, so X is not Sync
}