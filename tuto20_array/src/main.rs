fn main(){

    let numbers = [1,2,3,4,5];
    let numbers1:[i32;5] = [5,10,15,20,25];
    /*
        배열에서 자료형 부분이 생략되었지만 이게 background에서 돌아가고 있는거다.

    */
    let numbers2 = [2;10];
    /*
        배열에 값이 묶음으로 들어가는데,
        넣고싶은 값과 값의 갯수로 넣을 수 있다.
    */

    println!("numbers[0] : {}",numbers[0]);
    println!("numbers[4] : {}",numbers[4]);

    for n in numbers.iter(){
        println!("numbers : {} ",n);
        /*
         n자체가 numbers[n]에 해당하는 값이다.

        */


    }

    for i in 0..numbers1.len(){
        println!("numbers[{}] : {} ",i,numbers1[i]);
    }
    
    for i in 0..numbers2.len(){
        print!(" numbers2[{}] : {} ",i,numbers2[i]);
        if i % 5 == 0 {
            println!("");
        }
    }
}