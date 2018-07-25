extern crate rand;

/*
    not built-in which means you need to actually 
    use external crate in order to gain access to this functionality

    random are crates that you listed in your dependencies


*/

use rand::Rng;
/*that is trait part of this rand crate*/

fn main() {
    println!("Hello, world!");

    /*
        calling this thread_rng method which makes a random number generator from 
        thread_rng(min,max)
        from min to max can generate random number max-1
    */
    let random_number = rand::thread_rng().gen_range(1,11);

    println!("random number : {}",random_number);

    //Flip a coin
    /*
        이게 뭐지??
        Return a bool with a 1 in n chance of true
        1/5 의 확률로 true를 return 한다는 내용같은데??
        https://doc.rust-lang.org/1.1.0/rand/trait.Rng.html#method.gen_weighted_bool
    */
    let random_bool = rand::thread_rng().gen_weighted_bool(5);
    println!("random boolean : {} ",random_bool);
}
