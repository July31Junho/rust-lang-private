const MAX_NUM: u8 = 20;

fn main(){
    for n in 1..100{
        if n % 10 == 0 {
            println!("loop is {} ",n);
        }
        
    }
    println!("ConstMax :: {} ",get_const_max());
    // MAX_NUM = 30;
    // Error를 먹을것이다.
    

}

/**
 * 
 * warning: function `getConstMax` should have a 
 * snake case name such as `get_const_max`
 * 
 */
fn get_const_max() -> u8 {
    return MAX_NUM;
}


