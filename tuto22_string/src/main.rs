fn main() {
    println!("Hello, world!");

    let mut my_string = String::from("How's it going My name is Apple");

    println!("my_string is {} ",my_string);
    println!("my_string len is {} ",my_string.len());
    println!("my_string is empty ?{} ",my_string.is_empty());

    
    
    for token in my_string.split_whitespace(){

        println!("token : {}" ,token);
    } 


    println!("Does the string contain 'Apple'? :  {}", my_string.contains("Apple"));

    my_string.push_str("Hello Rust Sting");

    println!("{}",my_string);



}
