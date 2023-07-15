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
    let check: bool = test.find_in_text("LONGEST");

    println!("{}", check);
    
}















