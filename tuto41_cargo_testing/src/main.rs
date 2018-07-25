

struct Rectangle{
    width:u8,
    height:u8
}

impl Rectangle{
/*4 단계 
Rectangle이 square인지 아닌지를 테스트 해보겠다.
*/

    fn is_square(&self) -> bool {
        self.width == self.height
    }

}

fn main() {
    println!("Hello, world!");
}


/*3 단계 함수를 정의해서 return값 받기.*/
fn give_two() -> i32{
    2
}



/*1 단계*/
/*cargo test하기 전까지는 Compiled이 안된다.*/
#[cfg(test)]
mod print_tests{

    #[test]  /*testing function이라고 말한다. cargo에게 test function이라고 mark*/
    #[should_panic] /*panic이라고 예측을 했는데 Panic을 하면 성공한것이다.*/
    fn test_basic(){
        /*우리가 생각했을때 true 일거같은거를 넣어준다.*/
        assert!(1 == 1); //ok
        //cargo test를 하게되면 1pass가 뜰것이다.

        /*cargo test를 돌리면 panic상태로 들어갈것이고, 이것은 test의 실패이다.*/
        panic!("Oh no!");

    }
/*2 단계*/
    #[test]
    #[ignore] /*이렇게 해주면 이 함수는 테스트 하지 않을것이다.*/
    fn test_equals(){

        assert_eq!(2,1+1);
        /*
            두개의 파라미터가 서로 일치하는지 테스트
        */

        assert_ne!(2,1+2);

        /*
            두개의 파라미터가 서로 일치하는지 않은지 테스트
        */
    }

    #[test] 
    fn test_equals2(){
        /*3 단계*/
        /*위에 정의해놓은 함수를 호출해서 할 수 있다.*/
        assert_eq!(super::give_two(),1+1);
        assert_ne!(super::give_two(),1+2);
    }

    /*4 단계*/
    #[test]
    #[should_panic]  /*안에 있는 내용이 틀렸기에 Paninc 이라 fail을 내보낼꺼고 fail을 예상한거니깐 맞아서 test성공*/
    fn test_structs(){
        let r = super::Rectangle{
            width:50,
            height:25
        };
        /*여기서 순간 햇갈렸던게,
            구조체의 값을 대입하는 방법과, 구조체 tuple의 값을 대입하는 방법이 햇갈렸다.
            17강 18강에서 확인.
        */
        assert!(r.is_square());
        
    }
}