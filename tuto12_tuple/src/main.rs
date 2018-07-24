fn main(){

    let tup1 = (10,20,30,40);
    let tup2 = (1,2,"3","4",true,false);
    let tup3 = (1,2,"3","4",(1,2,3));
    println!("tuple : {}",tup1.0);

    println!("tuple : {}",(tup3.4).2);
    //println!("tup1.len() : {} ",tup1.len());

}