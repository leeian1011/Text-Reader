mod count_data;
use crate::count_data::CountData;
use std::fs::File;
use std::io::prelude::*;

fn main(){
    let mut file: File = File::open("src/files/test.txt").expect("Could not find file");
    let mut text: String = String::new();

    file.read_to_string(&mut text).expect("Could not read file");
}















