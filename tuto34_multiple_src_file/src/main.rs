mod module1;
/*Src file include */

/*

extern cate 와
mod 와
use 
이렇게 3가지의 차이가 있다.

*/


/*
#[allow(dead_code)]
Add a crate-level allow attribute; notice the !:
#![allow(dead_code)]
#![allow(unused)]


dead_code lint that will warn about unused functions
https://doc.rust-lang.org/rust-by-example/attribute/unused.html

A linter or lint refers to tools that analyze source code to flag programming errors, 
bugs, stylistic errors, and suspicious constructs.[1] 
The term originates from a Unix utility that examined C language source code.[2]
https://en.wikipedia.org/wiki/Lint_%28software%29
*/

fn main() {
    println!("Hello, world!");
    module1::print_message();


}
