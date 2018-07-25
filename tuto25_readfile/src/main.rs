use std::fs::File;
use std::io::prelude::*;


fn main(){

    let mut file = File::open("info.txt").expect("Can't open file!");

    /*expect 함수는 예상치 못한 일이 발생했을때 발동한다.*/

    // empty String
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Oops!.");


    println!("File Contents : \n\n{}",contents);
}