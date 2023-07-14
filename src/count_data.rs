use std::{fs::File, io::Read};

enum ExtractError {
    MissingFile,
    EmptyFile,
}

#[derive(Debug)]
pub struct CountData {
    pub text: String,
    char_count: u64,
    word_count: u64,
    sentence_count: u64,
}

impl CountData {
    fn extract_text(file_name: &String) -> Result<String, ExtractError> {
        let mut file: File = match File::open(file_name) {
            Ok(found_file) => found_file,
            Err(error) => return Result::Err(ExtractError::MissingFile),
        };

        let mut text: String = String::new();
        match file.read_to_string(&mut text) {
            Ok(string) => string,
            Err(error) => return Result::Err(ExtractError::EmptyFile),
        };
        //println!("successfully extracted {}", text);  DEBUG LINE 
        Ok(text)
    }
        
    pub fn new(file_name: &String) -> CountData {
        CountData{
            text: match CountData::extract_text(&file_name) {
                Ok(string) => string,
                Err(ExtractError::EmptyFile) => panic!("File is empty"),
                Err(ExtractError::MissingFile) => panic!("File does not exist"),
            },
            char_count: 0,
            word_count: 0,
            sentence_count: 0,
        }
    }
    
    pub fn set_char_count(&mut self, char_count: u64) -> (){
        self.char_count = char_count;
    }

    pub fn set_word_count(&mut self, word_count: u64) -> (){
        self.word_count = word_count;
    }

    pub fn set_sentence_count(&mut self, sentence_count: u64) -> (){
        self.sentence_count = sentence_count;
    }
}


