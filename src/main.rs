mod count_data;
use crate::count_data::CountData;
use std::fs::File;
use std::io::prelude::*;

fn main(){
    let s: String = String::from("src/files/test.txt");
    let test: CountData = CountData::new(&s);
}















