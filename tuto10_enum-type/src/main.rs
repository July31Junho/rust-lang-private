enum Direction{
    UP,
    DOWN,
    LEFT,
    RIGHT

}

fn main(){
    /*
     : 하나는 자료형이고
     :: 은 해당 클래스에 있는 값을 뜻함.
     */
    let player_direction:Direction = Direction::UP;

    //println!("Player_direction {} ",player_direction);


    match player_direction{
        Direction::UP => println!("We are heading up!"),
        Direction::DOWN => println!("We are heading down!"),
        Direction::LEFT => println!("We are heading left!"),
        Direction::RIGHT => println!("We are heading right!")
    }
}
