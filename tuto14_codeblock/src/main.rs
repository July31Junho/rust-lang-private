

fn main()
{   
    let x = 10;

    {
        //isolated
        println!(" x : {}",x);

        let y =200;
    }

    /**
     * Code가 Block별로 정의가 되기때문에 19번의 문장을 실행하게 될때는 error가 뜬다.
     * 
     */

        println!("y : {}",y);



}