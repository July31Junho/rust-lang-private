struct Color(u8,u8,u8);

fn main(){

    let red = Color(255,0,0);
    println!("red is {}, {}, {}",red.0,red.1,red.2);

    let mut red2 = Color(254,120,120);

    red2.0 = 10;

    println!("red2 is {}, {}, {} ",red2.0, red2.1, red2.2);
    

}
