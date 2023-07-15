# Simple Text-Reader in Rust

Currently transitioning from C to Rust, took some time to read the book and learn the massive differences between C and Rust.
Having just surface level understanding of OOP and no background in FP, I hope I'll be able to catch up to speed in the paradigms
although I believe I will probably have a much different view on the paradigms as I am learning them through Rust even though Rust does not
entirely practice either paradigms.

## Objectives in this Project

~~Hope to maybe learn more about how the ownership/borrowing memory system works by messing around with the `Strings` and `&str` types.
I also just learn better by building random stuff so hopefully this works out :p~~

- Found new objective(s):

1. Learn how to work with modules in my package, understand how to structure module when my package contains a binary crate.

2. Create independant module that main.rs uses in order to get count data out of text from a file.

3. Learn how to work within modules (understanding when to use `pub` and when not to!)

4. Work with the most beef'd up version of the `switch` statement. The `match` logic control flow. (It's INSANE)

5. Learn the insanely useful error-handling enum types `Option<_>` and `Result<T, E>`! 

6. Learn how command-line arguments are accepted in rust!

7. Learn how to use pattern-matching on command-line argument to ensure right type of file (.txt) is inputted requested.

## Notes

This programme will only work with .txt files and *ASCII* only text. I understand that rust implements UTF-8 encoding, but currently,
I would like to experiment with having all the expected read bytes be one byte big. Although, I think I could just use a `for x: char in
string' for loop to ease the issue, I'll try working with bytes first! (It's easier to match with byte literals)

Building this taught me simple usage of the `Result<T, E>` enum, a bit more about `Vec` and also just very slightly on how ownership and borrowing
works. Command-line arguments and also match control flow statement. All in all a pretty rounded basic project!

### Extras 
*I am horrible with creating digital mind map that captures my idea and concepts so here's one I've drawn out. I plan to learn how to use
software that allows me to create mind maps to better explain my concepts and ideas!*

![Image that has my rough concept jotted down](https://github.com/leeian1011/Text-Reader/blob/main/images/TextReaderConcept.jpeg)
