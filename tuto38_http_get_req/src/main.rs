extern crate reqwest;

fn main() {
    println!("Hello, world!");

    // match reqwest::get("http://www.google.com123651346345254"){
    //     Ok(mut response) =>{
    //         //check if 200 ok
    //         /*
    //             reqwest::StatusCode::Ok 이것도 Enum
    //             Ok면 성공적으로 Response를 얻을 수 있다.

    //         */
    //         if response.status() == reqwest::StatusCode::Ok{
    //             match response.text(){
    //                 Ok(text) => println!("Response Text: {}",text),
    //                 Err(_) => println!("Could not read response text!")
    //             }
    //         }else{
    //             println!("Response was not 200 OK.");
    //         }
    //     }
    //     Err(_) => println!("Could not make the request!")
    // }


    /*결과가 잘못되었다면 expect 명령어를 실행하게된다.
    text는 response text를 뜻하는거고, expect는 
    */
    let response_text = reqwest::get("http://youtube.com12356143223")
    .expect("Couldn't make request")
    .text().expect("Could not make the request!");


    println!("Request text : {}",response_text);
}
