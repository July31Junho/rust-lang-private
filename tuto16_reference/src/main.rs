fn main(){

    let mut x = 10;
    
    {
    let xr = &mut x;

    //let xrr = &mut x;
    //ampersand x mean 
    //you want to get a reference to the variable that
    *xr += 1;
    /**
     * 이 블락이 끝나면 xr 레퍼런스는 존재하지 않아서 
     * 문제가 되진 않는다!
     * 
     */
    }

    /**
     * error[E0502]: cannot borrow `x` as immutable because it is also borrowed as mutable
     * 
     */
    /**
     * 
     * 여기서 xr을 호출하려고 할때는 codeblock에 있어서 호출이 안된다.
     */
    println!("X : {0} , XR : ",x);
}