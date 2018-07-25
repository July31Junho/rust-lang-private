

struct Color{
    red:u8,
    green:u8,
    blue:u8
}

fn main()
{

    let blue = Color{red:0,green:10,blue:255};

    print_color(&blue);

    /*
        print_color(blue) 로 부르면
        blue를 2번 이상 호출시 error를 표현하게 된다.
        
    */
}

/*
    ref타입을 넘길땐,
    C언어에선

    void fucntionName(int *arr1, int *arr2);
    선언하고 넘겼었는데,

    int[255] arr;
    int[] arr = new int[]
    functionName(arr1,arr2);
    배열을 넘길땐 이렇게 했었고,

    변수 포인터를넘길땐

    functionName(int *a, int *b);
    functionName(&a, &b);
    이게 Rust에서는 조금 다르다.!!

*/

fn print_color(c: &Color)
{
    println!("Color = R:{}, G:{}, B:{}", c.red,c.green,c.blue);
}