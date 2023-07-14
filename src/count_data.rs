mod text_logic {
    use std::{fs::File, io::Read};

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
           match byte {
               b' ' | b'\n' => self.word_count += 1,
               b'.' => self.sentence_count += 1,
               _ => self.char_count += 1,
            };
        }

       self
    }
}



