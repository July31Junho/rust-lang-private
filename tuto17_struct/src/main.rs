struct Color{
    red:u8, //u8:0-255
    green:u8,
    blue:u8
}

fn main(){
    println!("Hello rust");


    let bg = Color{red:255, green:80, blue:15};
    //구조체에 value를 넣는 방법.alloc
    println!("{}, {}, {}",bg.red,bg.green,bg.blue);

    let mut bg2 = Color{red:255, green:80, blue:15};
    //구조체에 value를 넣는 방법.alloc

    bg2.red = 0;
    println!("{}, {}, {}",bg2.red,bg2.green,bg2.blue);

}