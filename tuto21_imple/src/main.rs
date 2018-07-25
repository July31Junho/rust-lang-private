

struct Rectangle{
    width:u32,
    height:u32
}


/*
    basically add methods to a struct to make it more useful
    imple

    bunch of cool stuff
*/

/*
    impl을 활용해서 struct에 함수를 넣는 방법
*/
impl Rectangle{
    fn print_description(&self){
        println!("Rectangle: {} x {} = {} ",self.width,self.height,self.width*self.height);
    }
    fn is_square(&self) -> bool{
        self.width == self.height
    }
}
fn main() {
    println!("Hello, world!");

    let my_rect = Rectangle{width:10,height:5};

    my_rect.print_description();

    println!("Rectangle is a square : {}", my_rect.is_square());
}
