fn main() {
    println!("Hello, world!");

    let number = 10;

    match number {
        1 => println!("It is one!"),
        2 => println!("There is two of them!"),
        10 | 11 => println!("any number in the array"),
        _ => println!("There is nothing here ")
    }


    let name = "Rainbow";

    match name {
        "ApplePie" => println!("ApplePie"),
        "Rainbow" => println!("Rainbow"),
        "Ground Dog" => println!("Ground Dog"),
        _ => println!("Anythin else")

    }
}
