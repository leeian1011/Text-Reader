use std::fs::File;
use std::io::prelude::*;

fn main(){
    let mut test: File = File::open("src/text.txt").expect("err file");
    let mut s1: String = String::new();
    
    test.read_to_string(&mut s1).expect("could not read");

    println!("{}", s1);
}
