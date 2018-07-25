#![allow(dead_code)]

enum Day {
    Monday,Tuesday, Wednesday, Thursday, Friday,
    Saturday,Sunday
}


impl Day {
    /*여기서 구현되는 함수들은 모두 Day Enum의 함수들이 된다.*/
    fn is_weekday(&self) -> bool {
        match self {
            /*&self 와 맞추기 위해서
            reference type으로 값을 비교한다.get no compile error
            */
            &Day::Saturday | &Day::Sunday => return false,
            _ => return true
        }
    }

}
fn main() {
    println!("Hello, world!");
    let d = Day::Tuesday;


    println!("Is d a weekday? {} ", d.is_weekday() );
}
