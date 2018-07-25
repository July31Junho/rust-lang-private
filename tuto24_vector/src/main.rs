use std::result::Result;

pub type Result<T> = ::std::result::Result<T, Box<TrieError>>;

fn main() -> Result<(), String>{
    

    let mut vector:Vec<i32> = Vec::new();

    vector.push(2);
    vector.push(4);
    vector.push(6);
    vector.push(7);
    
    for number in vector.iter(){
        println!("{}",number);
    }

    //백터를 이렇게도 정의할 수 있고,
    let mut my_vector = vec![2,4,6,8];

    my_vector.push(50);
    my_vector.remove(1);


    for number in my_vector.iter(){
        println!("{}",number);
    }


}