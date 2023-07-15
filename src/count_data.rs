use std::ops::Index;

mod text_logic {
    use std::{fs::File, io::Read};
    pub const WHITE_SPACES: [u8; 3] = [b' ', b'\n', b'\t'];
    pub const PUNCTUATION: [u8; 3] = [b'.', b'!', b'?'];
    pub const NON_CHARACTERS: [u8; 3] = [b'\'', b'"', b';'];

    pub enum ExtractError {
        MissingFile,
        EmptyFile,
    }

    pub fn extract_text(file_name: &String) -> Result<String, ExtractError> {
        let mut file: File = match File::open(file_name) {
            Ok(found_file) => found_file,
            Err(_) => return Result::Err(ExtractError::MissingFile),
        };

        let mut text: String = String::new();
        match file.read_to_string(&mut text) {
            Ok(string) => string,
            Err(_) => return Result::Err(ExtractError::EmptyFile),
        };
        //println!("successfully extracted {}", text);  DEBUG LINE 
        Ok(text)
    }
}

#[derive(Debug)]
pub struct CountData {
    pub text: String,
    char_count: u64,
    word_count: u64,
    sentence_count: u64,
}

impl CountData {
    pub fn new(file_name: &String) -> CountData {
        CountData{
            text: match text_logic::extract_text(&file_name) {
                Ok(string) => string,
                Err(text_logic::ExtractError::EmptyFile) => panic!("File is empty"),
                Err(text_logic::ExtractError::MissingFile) => panic!("File does not exist"),
            },
            char_count: 0,
            word_count: 0,
            sentence_count: 0,
        }
    }
    
    pub fn count(mut self) -> CountData{
       let text_bytes: &[u8] = self.text.as_bytes();

       for &byte in text_bytes {
           if text_logic::WHITE_SPACES.contains(&byte) {
               self.word_count += 1;
           }else if text_logic::PUNCTUATION.contains(&byte) {
               self.sentence_count += 1;
           }else if !text_logic::NON_CHARACTERS.contains(&byte) {  //contemplating whether to use//!text::logic... or  //else if continue;
               self.char_count += 1;
           } 
        }

       self
    }

    pub fn level(&self) -> f32 {
        //This uses the Coleman-Liau Index Formula
        let letter_per_word: f32 = (self.char_count as f32)/(self.word_count as f32);
        let sentence_per_word: f32 = (self.sentence_count as f32)/(self.word_count as f32);        
        
        let average_letters = letter_per_word * 100.0;
        let average_words = sentence_per_word * 100.0;
        0.0588 * average_letters - 0.296 * average_words - 15.8
    }
    
    pub fn find_in_text(&self, pattern: &str) -> bool {
        let pattern_bytes: &[u8] = pattern.as_bytes();
        let text_bytes: &[u8] = self.text.as_bytes();
        let mut word_bytes: Vec<u8> = Vec::new();

        for &byte in text_bytes {
            if byte == b' '{
                if word_bytes.len() != pattern_bytes.len() {
                    word_bytes.clear();
                    continue;
                }

                for i in 0..pattern.len() {
                    if word_bytes[i] != pattern_bytes[i] {
                        word_bytes.clear();
                        break;
                    }
                    println!("{}", word_bytes[i] as char);
                    if i == pattern.len() - 1 {
                        return true
                    }
                }
            }

            word_bytes.push(byte.clone());
        }
        
        false
    }
}



