fn main() {
    println!("Hello, world!");

    /*Replace*/
    {
        /*String from 과 String new는 다른 관점이다.
        String new : https://doc.rust-lang.org/std/string/struct.String.html#method.new
        String from : https://doc.rust-lang.org/std/string/struct.String.html
        */
        let my_string = String::from("Rust is fantastic!");
        println!("After replace {}",my_string.replace("fantastic","great"));
    }

    /*Lines*/

    {

        let my_string = String::from("The weather is \nnice \noutside mate!.");


        for line in my_string.lines(){

            println!("[{}]",line);
        }
    }


    /*Split*/
    {
        let my_string = String::from("Leave+a+like+if+you+enjoyed!");
        let tokens:Vec<&str> = my_string.split("+").collect();

        println!("{}",my_string);
        println!("At index 2: {}",tokens[2]);
    }

    /*Trim*/

    {
        let my_string = String::from("      My name is Split      \n\r");

        println!("Before trim : {} ",my_string);
        println!("After trim : {}",my_string.trim());

    }

    /*Chars*/

    {
        let my_string = String::from("dcode on Youtube");
        /*Get character at index*/
        println!("Before Chars : {} ",my_string);

        match my_string.chars().nth(4){
            Some(c) => println!("Character at index 4:{}",c),
            None => println!("No character at index 4.")
        }
    }
}
 