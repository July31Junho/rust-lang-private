
fn main(){

    let name = String::from("ABCDEFGHIJ");

    /*이럴경우 10개의 letter가 있기 때문에 11번째의 letter에 접근은 불가능하다.
    */
 
 /*   println!("Character at index 8: {}", match name.chars().nth(8){
        Some(c) => println!("index 8 : {}",c),
        None => println!("There's no value")
        });

        use std::fmt;


impl fmt::Display {
    
}

        */

        println!("character at index 8 : {}" , match name.chars().nth(8){
            Some(o) => o.to_string(),
            None => "No character at index 8!".to_string()
        });

        println!("{}",name);
        //println!("{}",name.chars());
        println!("Occupation is {} ",match get_occupation("Domenic"){
            /*
            Some에서의 c || o 는 Option의 result로 저거 스스로를 뜻한다.
            itself
            */
            Some(c) => c,
            None => "No occpation found!"
        });


}

/*Option type을 리턴하는걸 살펴보겠다.

Option<&str>
Option enum은 string을 준다.
if you get value from this funtion
it's gonna be off type string 
this option enum is going to give us a string
if the some member is returned if that makes sense 

*/
fn get_occupation(name:&str) ->Option<&str>{
    match name {
        "Domenic" => Some("Software Developer"),
        "Michael" => Some("Dentist"),
        _=> None
    }

}