use std::env;

fn main(){

    let args:Vec<String> = env::args().collect();

    for argument in args.iter(){

        println!("argument : {} ", argument);
    }

    /*
        첫번째 파라미터는 index 0 는 path다 Execut able 한 파일의
    */
    println!("args[1] = {}",args[1]);
}