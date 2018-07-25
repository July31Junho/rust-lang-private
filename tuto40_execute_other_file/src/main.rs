use std::process::Command;


fn main() {
    
    //python print.py

    let mut cmd = Command::new("python");
    cmd.arg("print.py");

    //Execute the command

    match cmd.output(){
        Ok(o) => {
            unsafe{
                /*  
                    String::from_utf8_unchecked 이 함수 자체가
                    안전하지 않아서 unsafe block으로 안아준다.
                    utf-8인지 valid한지 check를 안해서.
                */
                println!("output : {}",String::from_utf8_unchecked(o.stdout));
            }
            //do stuff here
            //o.stdout  은 string이 아니고 Vectorized가 된거여서 아래처럼 파싱을 해줘야한다.
            

        },
        Err(e) => {
            println!("There was an error! {}",e);
        }
    }



}
