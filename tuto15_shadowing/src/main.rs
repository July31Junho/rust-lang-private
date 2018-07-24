/**
 * 
 * Shadowing is sort of like reassigning a variable
 * 
 */

fn main() {
    

    let mut x = 200;


    {

        let mut x= 10;
    }

    x = 20;

    println!("X : {} ", x);
}
