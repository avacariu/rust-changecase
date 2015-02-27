#![feature(unicode)]

#[derive(Copy)]
pub enum Case {
    Upper,
    Lower
}

pub trait ChangeCase {
    fn to_uppercase(&self) -> Self;
    fn to_lowercase(&self) -> Self;
    fn to_capitalized(&self) -> Self;
    fn to_invertedcase(&self) -> Self;
    fn to_alternatingcase(&self, start_with: Case) -> Self;
}

impl ChangeCase for String {
    fn to_uppercase(&self) -> String {
        if self.is_empty() {
            return String::new();
        }

        let mut result = String::with_capacity(self.len());

        {
            for c in self.chars() {
                result.push(c.to_uppercase());
            }
        }

        result
    }

    fn to_lowercase(&self) -> String {
        if self.is_empty() {
            return String::new();
        }

        let mut result = String::with_capacity(self.len());

        {
            for c in self.chars() {
                result.push(c.to_lowercase());
            }
        }

        result
    }

    fn to_capitalized(&self) -> String {
        if self.is_empty() {
            return String::new();
        }

        let mut result = String::with_capacity(self.len());

        {
            let mut chars = self.chars();
            result.push(chars.next().unwrap().to_uppercase());

            for c in chars {
                result.push(c.to_lowercase());
            }
        }

        result
    }

    fn to_invertedcase(&self) -> String {
        if self.is_empty() {
            return String::new();
        }

        let mut result = String::with_capacity(self.len());

        {

            for c in self.chars() {
                if c.is_uppercase() {
                    result.push(c.to_lowercase());
                } else if c.is_lowercase() {
                    result.push(c.to_uppercase());
                } else {
                    result.push(c);
                }
            }
        }

        result
    }

    fn to_alternatingcase(&self, start_with: Case) -> String {
        if self.is_empty() {
            return String::new();
        }

        let mut result = String::with_capacity(self.len());

        {
            let mut current_case = start_with;

            for c in self.chars() {
                if c.is_uppercase() || c.is_lowercase() {
                    match current_case {
                        Case::Upper => {
                            result.push(c.to_uppercase());
                            current_case = Case::Lower;
                        },
                        Case::Lower => {
                            result.push(c.to_lowercase());
                            current_case = Case::Upper;
                        }
                    };
                } else {
                    result.push(c);
                }
            }
        }

        result
    }
}

#[test]
fn upper() {
    assert_eq!(String::from_str("Something").to_uppercase(), "SOMETHING");
    assert_eq!(String::from_str("sOmEtHiNg").to_uppercase(), "SOMETHING");
    assert_eq!(String::from_str("word word2").to_uppercase(), "WORD WORD2");
}

#[test]
fn lower() {
    assert_eq!(String::from_str("SOMETHING").to_lowercase(), "something");
    assert_eq!(String::from_str("sOmEtHiNg").to_lowercase(), "something");
    assert_eq!(String::from_str("Word WORD2").to_lowercase(), "word word2");
}

#[test]
fn capitalize() {
    assert_eq!(String::from_str("SOMETHING").to_capitalized(), "Something");
    assert_eq!(String::from_str("sOmEtHiNg").to_capitalized(), "Something");
    assert_eq!(String::from_str("Word WORD2").to_capitalized(), "Word word2");
}

#[test]
fn swapped() {
    assert_eq!(String::from_str("SOMETHING").to_invertedcase(), "something");
    assert_eq!(String::from_str("SoMeThInG").to_invertedcase(), "sOmEtHiNg");
    assert_eq!(String::from_str("Some WorD2").to_invertedcase(), "sOME wORd2");
}

#[test]
fn alternating() {
    assert_eq!(String::from_str("SOMETHING").to_alternatingcase(Case::Lower), "sOmEtHiNg");
    assert_eq!(String::from_str("SOMETHING").to_alternatingcase(Case::Upper), "SoMeThInG");
    assert_eq!(String::from_str("som3thing").to_alternatingcase(Case::Lower), "sOm3ThInG");
    assert_eq!(String::from_str("som3thing").to_alternatingcase(Case::Upper), "SoM3tHiNg");
    assert_eq!(String::from_str("some word").to_alternatingcase(Case::Lower), "sOmE wOrD");
}
