/**
 * input command line
 * Help page
 * port-sniffer -h
 * The number of Thread num  
 * port-sniffer -j 100 192.168.0.1
 * Default Tread num setting
 * port-sniffer 192.168.0.1
 * 
 * 
 */
use std::env;
use std::io::{self,Write};
use std::net::{IpAddr,TcpStream};
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender,channel};
use std::thread;

const MAX: u16 = 65535;


//arguments로 받은 파라미터들을 더 효율적으로 관리하기 위해서
struct Arguments {

    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

/*Arguments에 필요한 함수 구현.*/
impl Arguments {
    fn new(args: &[String])-> Result<Arguments, &'static str>{
        if args.len() < 2{
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("Too many arguments");
        }

        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f){
            return Ok(Arguments {flag: String::from(""), ipaddr,threads:4});
        }else{
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2{
                println!("Usage: -j to select how many threads you want
                \n\r       -h or -help to show this help message");
                return Err("help");
            }else if flag.contains("-j"){
                let ipaddr = match IpAddr::from_str(&args[3]){
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IPADDR; must be IPv4 or IPv6")
                };
                let threads = match args[2].parse::<u16>(){
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse thread number")
                };
                return Ok(Arguments{threads, flag, ipaddr});
            } else {
                return Err("invalid syntax");
            }
        }
    }
}


fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16){
    let mut port: u16 = start_port + 1;
    loop{
        match TcpStream::connect((addr,port)){
            Ok(_) =>{
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    /* input으로 들어오는 arguments를 구조체에 매칭해서 데이터를 넣어주는거. */
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help"){
                process::exit(0);
            }else{
                eprintln!("{} problem parsing arguments: {} ",program,err);
                process::exit(0);
            }
        }
    );
    /*
    arguments test
    for i in &args{
        println!("{}",i);
    }
    println!("{:?}",args);
    

    let program = args[0].clone();
    let flag = args[1].clone();
    let threads = args[2].clone();
    let ipaddr = args[3].clone();
    */

    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;
    let (tx,rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx,i,addr,num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }
    println!("");
    out.sort();
    for v in out {
        println!("{} is open ",v);
    }
}
