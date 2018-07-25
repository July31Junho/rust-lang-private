//const MAX_NUM: u8 = 20;
const NUM:i32 = 20;


fn main(){

    /*
        배열을 어떻게 써야하는지, 정수형 type이 안 맞다는 메세지가 나오는데
        어떻게 해야하지

        흠..


        
    */
    //let numbers1:[i32;5] = [5,10,15,20,25];
    let mut arr1:[i32;5] = [5,10,15,20,25];
    let mut arr2 = [5,4,3,2,1];

    let mut arr3 = [0;NUM];


    println!("초기값은 : ");
    
    for i in 0..arr1.len(){
        print!("arr2[{}] : {} ",i,arr1[i]);
    }

    /*
    for i in 1..NUM-1{
        for j in 1..NUM-1{
            println!("{} X {} = {}",i,j,i*j);
        }
    }

    for i in 0..NUM-1{
        for j in 0..NUM-1{
            if num
        }
    }
    */
    println!("{}",arr3[2]);
}