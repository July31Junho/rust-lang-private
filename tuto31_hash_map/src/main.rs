/*
    import Hash-Map struct
*/
use std::collections::HashMap;

fn main(){
    let mut marks = HashMap::new();

    /* 
        Insert values onto this hash map by using the insert method
    */
    // Add values
    marks.insert("Rust Programming",96);
    marks.insert("Web Development",80);
    marks.insert("UX Design",70);
    marks.insert("Network Programming",60);


    //Find length of HashMap
    println!("How many subjects have you studied? {}",marks.len());


    /*
        2가지를 알 수 있는데
        첫 번째는 키가 Web Development인 벨류인 점수를 알 수 있고,
        두 번째는 키가 Web Development가 존재하는지를 알 수 있다.

    */
    match marks.get("Web Development") {
        Some(mark) => println!("You got {} for Web Dev!",mark),
        None => println!("you couldn't studied web programming!")
    }



    //Remove a value

    marks.remove("UX Design");

    for (subject, mark) in &marks{

        println!("For {}, you got {}%!",subject,mark);
    }

    //check for value

    println!("Did you stdy C++ {} ",marks.contains_key("C++"));
}
