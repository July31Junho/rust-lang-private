use std::io;

fn main(){

    let mut input = String::new();

    println!("Hey mate Say Something:");

    /*
        io::stdin().read_line(&mut input) => return result
        result could be either a Success or Failure

        using match state
        I can see what happens when i was okay and 
        what happens when there was an error so


    */
    match io::stdin().read_line(&mut input){
        
        /* under scope will ignore that variable*/
        Ok(_) => {
            println!("Success! you said : {}",input);
            println!("Success! you said : {}",input.to_uppercase());
        },
        Err(e) => println!("Error! Something went wrong: {} ",e)


    }

}