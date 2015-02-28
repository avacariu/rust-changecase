# rust-changecase [![Build Status](https://travis-ci.org/vlad003/rust-changecase.svg)](https://travis-ci.org/vlad003/rust-changecase)

Change the case of Strings in Rust.

Available functions:

* Uppercase
* Lowercase
* Invert case (inverts the case of each character)
* AlTeRnAtInG cAsE

What I'm working on:

* Title Case
* Implementation for `&str`

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
