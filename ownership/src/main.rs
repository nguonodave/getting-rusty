// fn main() {
//     let mut s = String::from("hey");
//     s.push_str("ch");

//     // // this will give an error since s1 was moved to s2, hence s1 is no longer valid
//     // let s1 = String::from("hello");
//     // let s2 = s1;

//     // println!("{s1}, world!");

//     // // this is cloning, which works like deep copying
//     // // the values in the stack and heap of s1 are all allocated to s2
//     // let s1 = String::from("hello");
//     // let s2 = s1.clone();

//     // println!("s1 = {s1}, s2 = {s2}");

//     // this is okay since ints are stored in the stack cause they have a known size
//     let x = 5;
//     let y = x;

//     println!("x = {x}, y = {y}");
// }

// fn main() {
//     let s = String::from("hello"); // s comes into scope

//     takes_ownership(s); // s's value moves into the function...
//                         // ... and so is no longer valid here

//     let x = 5; // x comes into scope

//     makes_copy(x); // x would move into the function,
//                    // but i32 is Copy, so it's okay to still
//                    // use x afterward
// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
//   // special happens.

// fn takes_ownership(some_string: String) {
//     // some_string comes into scope
//     println!("{some_string}");
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) {
//     // some_integer comes into scope
//     println!("{some_integer}");
// } // Here, some_integer goes out of scope. Nothing special happens.

// fn main() {
//     let s1 = gives_ownership(); // gives_ownership moves its return
//                                 // value into s1

//     let s2 = String::from("hello"); // s2 comes into scope

//     let s3 = takes_and_gives_back(s2); // s2 is moved into
//                                        // takes_and_gives_back, which also
//                                        // moves its return value into s3
// } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
//   // happens. s1 goes out of scope and is dropped.

// fn gives_ownership() -> String {
//     // gives_ownership will move its
//     // return value into the function
//     // that calls it

//     let some_string = String::from("yours"); // some_string comes into scope

//     some_string // some_string is returned and
//                 // moves out to the calling
//                 // function
// }

// // This function takes a String and returns one
// fn takes_and_gives_back(a_string: String) -> String {
//     // a_string comes into
//     // scope

//     a_string // a_string is returned and moves out to the calling function
// }

// Borrowing - passing in references as function parameters. borrowing the value instead of actually taking ownership
// fn main() {
//     let s1 = String::from("hey");
//     let len = get_len(&s1);
//     println!("{len}")
// }

// fn get_len(s: &String) -> usize {
//     s.len()
// }

// // to be able to mutate without taking ownership, we do:
// fn main() {
//     let mut s1 = String::from("hey"); // make s1 a mutable var
//     change(&mut s1);
//     println!("{s1}");

//     let s2 = &mut s1;
//     println!("{}", *s2);
// }

// fn change(s: &mut String) {
//     // // to change, you can clear then push a new str
//     // s.clear();
//     // s.push_str("hi");

//     // OR dereference the value of the reference and assign a new value
//     *s = String::from("there")
// }

// // Dangling references - references that point to invalid data
// fn main() {
//     let cc = gg();
// }

// // in this &s is a dangling reference (hence an error) because after the function ends
// // s is defined within the scope of gg() so when gg() ends s will be dropped, it will be deallocated from the heap
// // meaning the returned reference will be pointing to an invalid memory since the memory no longer exists from the heap
// // the alternative option is returning the string instead of the reference to the string
// fn gg() -> &String {
//     let s = String::from("hey");
//     &s
// }

fn main() {
    println!("we will continue from here")
}
