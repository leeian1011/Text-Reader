mod count_data;
use crate::count_data::CountData;
use std::env;

fn main(){
    let argv: Vec<String> = env::args().collect();
    if argv.len() != 2 {
        println!("Usage: cargo run [src/files/example.txt]");
        return
    }
    
    let test: CountData = CountData::new(&argv[1]).count();
    
   println!("{:?}", test); //DEBUG LINE
    println!("{}", test.level());
}















