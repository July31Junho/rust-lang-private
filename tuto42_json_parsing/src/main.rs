/*
json을 파싱하는 방법엔 2가지가 있다.
1st the data through an array like syntax
2nd the json data to and actual struct 
*/


/*외부 라이브러리 json crate를 사용할것이다.*/
extern crate serde_json; /*이게 첫 번째 방법이였고,*/
use serde_json::Value as JsonValue; /*json struct 이다. 별칭을 줘서 사용*/


extern crate serde;
#[macro_use]
extern crate serde_derive;

/*Struct를 하나 만들꺼고 이게 template이 될것이다.
*/
#[derive(Serialize,Deserialize)]
struct Person{
    name: String,
    age: u8,
    is_male:bool
}


fn main() {
    println!("Hello, world!");

    let json_str = r#"
        {
            "name":"user1",
            "age":62,
            "is_male":true

        }
    "#;

    let res = serde_json::from_str(json_str);

    if res.is_ok(){
        let p:JsonValue = res.unwrap();

        println!("The name is {}" ,p["name"]);
        println!("The name is {}" ,p["name"].as_str().unwrap());
        /*이런 방법도 있지만. Option이고 추천하진 않는다. 
        "" 이 없는 str을 얻을 수 있다.
        */

    }else {
        println!("Sorry! Could not parse Json: ");
    }

    println!("Json Second how");

    /*2 번째 방법*/
    let json_str2 = r#"
        {
            "name":"user2",
            "age":52,
            "is_male":true
        }
    "#;
    let res2 = serde_json::from_str(json_str2);
    if res2.is_ok(){
        let p2:Person = res2.unwrap();
        println!("The name is {} ",p2.name);
        println!("The age is {} ",p2.age);
        println!("Is male ? :{} ",p2.is_male);
    } else{
        println!("Sorry! Could not parse Json2 ");

    }


}
/*json 파싱이라는건
pass or decode json 
json string을 serialized해서 사용할 수 있는 data structure로 convert를 한다.
*/