const MAXNUM : i64 = 250;

fn main() {

    let tup8 = (10,20,30,40);
    let (a,b,c,d) = tup8;
    println!("tup8 : {}", tup8.2);
    println!("a : {}", a);
    println!("b : {}", b);
    println!("c : {}", c);

    for i in 200..MAXNUM {
        if i % 5 == 0 {
            println!(" i : {}",i);
        }
    }


}
