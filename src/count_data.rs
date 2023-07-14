pub struct CountData{
    char_count: u64,
    word_count: u64,
    sentence_count: u64,
}

impl CountData {
    pub fn new() -> CountData {
        CountData{
            char_count: 0,
            word_count: 0,
            sentence_count: 0,
        }
    }
    
    pub fn set_char_count(&mut self, char_count: u64) -> (){
        self.char_count = char_count;
    }

    pub fn set_word_count(&mut self, word_count: u64) -> (){
        self.char_count = word_count;
    }

    pub fn set_sentence_count(&mut self, sentence_count: u64) -> (){
        self.sentence_count = sentence_count;
    }

}


