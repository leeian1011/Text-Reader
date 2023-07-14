# Learning Rust
- Coming from a C background, learning the ownership & borrowing system was not the most difficult ordeal (I have yet to work with it
in convoluted codebases). The idea of borrowing makes sense and referencing/slice typing is pretty intuitive (so far :) ).

- I haven't really taken the time to transition to a different language from C, so I have never felt the neutron activation when I realized
some of the logic control flow are pretty much identical. `if` statements work the same (this is literal true for all languages LMAO).
I made the connection that `match` is literally a `switch` statement on PEDs.
Result<T, E> is just sentinel value that communicates much more clearly. 
Option<T> is just a jacked up `if (x == NULL)` that ensures you cover both cases if `x` is or is not `NULL`.
Honestly `match`, `Result<T, E>` and `Option<T>` has made me really really enjoy learning Rust so far.

## What I aim to learn building this

- Attempt to use Result<T, E> in my code. (Implemented in count_data.rs)

- Attempt to use Option<T> in my code. (Pending)

- Learning how to structure modules with a binary crate root.

- Learning how to properly structure modules with the use of `pub`

- Learning method associated vs associated functions.

## Things I've learnt so far

- Result<T, E>
This enum is amazing, it makes it much more easier to communicate and read that an error could occur in a function.
> This is probably caused from the lack of using the enum type in C whilst I was learning C honestly.

I learnt that the `match` statement can be used on functions.
Learnt how to assign a return value from an enum<T> using a match statement.
```rust
let result_example: Result<String, GenericError> = Ok(String::from("ye"));
let mut example: String = match result_example {
    Ok(string) => string,
    Err(err) => panic!("yuh"),
};
```
>Rough example on how I learnt this!
