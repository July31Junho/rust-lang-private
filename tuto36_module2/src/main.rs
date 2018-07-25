mod dcode{

    fn chicken(){

        println!("Chicken!");
    }
    pub fn print_message(){

        chicken();
        println!("How's it going!");
    }
    /*기본 접근 제어자가 private 였기 때문에,*/

    pub mod water{

        pub fn print_message(){
            println!("Water print_message");
        }
    }
}

fn main(){
    dcode::print_message();


    dcode::water::print_message();

}