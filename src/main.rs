mod count_data;
use crate::count_data::CountData;
use std::fs::File;
use std::io::prelude::*;

fn main(){
    let s: String = String::from("src/files/test.txt");
    let mut test: CountData = CountData::new(&s);
    test.set_char_count(100);
    test.set_word_count(20);
    test.set_sentence_count(2);
    println!("{:?}", test);
}















