fn main(){

    let numbers =30..51; 
    /*
    for i in 1..11{
        println!("The number is {}",i);
    }
    */

    for i in numbers {
        if i % 5 == 0 {
            println!("The number is {}",i);
        }
    }
    let animals = vec!["Rabbit","Dog","Cat"];

    for a in animals.iter(){
        println!("The animal name is {}",a);

    }

    /*사용하고 있는 인덱스를 알고싶을 때 */
    for (index,a) in animals.iter().enumerate(){

        println!("the Index is {} and the animal name is {}",index,a);
    }

}