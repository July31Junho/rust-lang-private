extern crate regex;
/*dependency list에 있는 crate를 사용할때 extern crate 를 이용해서 넣어주고.*/
use regex::Regex;

/*사용할때는 이렇게 사용한다.*/

/*이런거는 이제 Random에서도 해봤었지.
*/


/*
reg는 잘 이해가 안갔다.
*/
fn main() {
    println!("Hello, world!");


    /* 
        r""  : raw string
        \d reg expression을 말한다. ( single digit을 뜻)
        \w{5} two curly braces and five 하면 5word를 reg expression하게 된다.
        5 letter word
        Regex::new() 만 사용해서 하면 Invalid regex를 넘겼을때 Error가 나오게 된다.

        safty 사용방법 unwrap() 함수


    
    */
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let text = "2012-03-14, 2013-01-01 and 2014-07-05";
    
    for cap in re.captures_iter(text) {
        println!("Month: {} Day: {} Year: {}", &cap[2], &cap[3], &cap[1]);
    }

    let re1 = Regex::new(r"\w{5}").unwrap();
    let text = "abc";

    println!("Found match? {}",re1.is_match(text));


    match re1.captures(text){
        Some(caps) => println!("Found match : {}",caps.get(0).unwrap().as_str()),
        None => println!("Could not find match...")

    }

}
/*
    정규식을 사용하려면 먼저 Regex라는 Dependency를 toml파일에 넣어줘야한다.

*/