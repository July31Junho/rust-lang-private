struct Person{
    name:String,
    age:u8
}

/*
    trait is like a certain set of rules or requirements that 
    a object or struct must have in order to have that name the trait
    if that makes sense so it's better if i go by example.
    if we define our new struct up here
*/
trait HasVoiceBox {
    //Speak
    fn speak(&self);
    //check if can speak
    fn can_speak(&self) -> bool;
}

/*
    defining area
*/
impl HasVoiceBox for Person{
    fn speak(&self){
        println!("Hello, my name is {}", self.name);
    }


    fn can_speak(&self) -> bool{
        if self.age > 0 {
            return true;
        } else {
            return false;
        }

    }
}

fn main() {
    println!("Hello, world!");


    /*
        (
        )
            괄호와 중괄호는 주의를 하기.
        {
                
        }
    */
    let person = Person{name:String::from("Bob"), age:41};

    println!("Can {} speak? {}" ,person.name,person.can_speak());


    let person2 = Person{name:String::from("Alice"), age:0};
    println!("Can {} speak? {}",person2.name,person2.can_speak());
}
