# rust-changecase [![Build Status](https://travis-ci.org/avacariu/rust-changecase.svg)](https://travis-ci.org/avacariu/rust-changecase)

Change the case of Strings in Rust. It's also implemented for `&str`. In the
latter case, it returns a String.

Available functions:

* Uppercase
* Lowercase
* Invert case (inverts the case of each character)
* AlTeRnAtInG cAsE

What I'm working on:

* Title Case

**Note:** The implementation for `&str` returns String. I'd definitely prefer
for it to return `&str` but I noticed that `std::ascii:AsciiExt` also returns a
String for `&str`, so I went with that. I'm also not really sure how to get the
functions to return `&str` :confused:

## Examples

    extern crate changecase;
    use changecase::ChangeCase;
    use changecase::Case;

    assert_eq!(String::from_str("Some").to_uppercase(), "SOME");
    assert_eq!(String::from_str("Some").to_lowercase(), "some");
    assert_eq!(String::from_str("Some").to_invertedcase(), "sOME");
    assert_eq!(String::from_str("some thing").to_capitalized(), "Some thing");
    assert_eq!(String::from_str("Some").to_altcase(Case::Lower), "sOmE");
    assert_eq!(String::from_str("Some").to_altcase(Case::Upper), "SoMe");

    assert_eq!("Some".to_uppercase(), "SOME");
    assert_eq!("Some".to_lowercase(), "some");
    assert_eq!("Some".to_invertedcase(), "sOME");
    assert_eq!("some thing".to_capitalized(), "Some thing");
    assert_eq!("Some".to_altcase(Case::Lower), "sOmE");
    assert_eq!("Some".to_altcase(Case::Upper), "SoMe");
