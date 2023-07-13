use std::fs::File;
use std::io::prelude::*;

fn main(){
    let mut test: File = File::open("../../src/text.txt").expect("err file");
}
