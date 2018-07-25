use std::fs::File;
use std::io::prelude::*;
/*io::prelude 는 io관련 무거운 모듈을 완화하는 모듈이라고 보면 될것같다.
https://doc.rust-lang.org/std/io/prelude/
*/


fn main(){

    /*
        줄바꿈을 했을때 궁금한게 있엇는데, shift enter를 

    */
    let mut file = File::create("output.txt")
        .expect("Could not create file");

    file.write_all(b"Welcome to dcode!")
    .expect("cannot write to the file, Sorry mate.");
}