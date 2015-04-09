#![feature(collections)]

#[derive(Copy, Clone)]
pub enum Case {
    Upper,
    Lower
}

pub trait ChangeCase {
    fn to_uppercase<'a>(&'a self) -> String;
    fn to_lowercase<'a>(&'a self) -> String;
    fn to_capitalized<'a>(&'a self) -> String;
    fn to_invertedcase<'a>(&'a self) -> String;
    fn to_altcase<'a>(&'a self, start_with: Case) -> String;
}

impl<'a> ChangeCase for String {
    fn to_uppercase(&self) -> String {
        if self.is_empty() {
            return String::new();
        }

        let mut result = String::with_capacity(self.len());

        {
            for c in self.chars() {
                let upper = c.to_uppercase().next().unwrap();
                result.push(upper);
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
                let lower = c.to_lowercase().next().unwrap();
                result.push(lower);
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
            let first = chars.next().unwrap().to_uppercase().next().unwrap();
            result.push(first);

            for c in chars {
                let lower = c.to_lowercase().next().unwrap();
                result.push(lower);
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
                    let lower = c.to_lowercase().next().unwrap();
                    result.push(lower);
                } else if c.is_lowercase() {
                    let upper = c.to_uppercase().next().unwrap();
                    result.push(upper);
                } else {
                    result.push(c);
                }
            }
        }

        result
    }

    fn to_altcase(&self, start_with: Case) -> String {
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
                            let upper = c.to_uppercase().next().unwrap();
                            result.push(upper);
                            current_case = Case::Lower;
                        },
                        Case::Lower => {
                            let lower = c.to_lowercase().next().unwrap();
                            result.push(lower);
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

impl<'a> ChangeCase for &'a str {
    fn to_uppercase(&self) -> String {
        String::from_str(self).to_uppercase()
    }
    fn to_lowercase(&self) -> String {
        String::from_str(self).to_lowercase()
    }
    fn to_capitalized(&self) -> String {
        String::from_str(self).to_capitalized()
    }
    fn to_invertedcase(&self) -> String {
        String::from_str(self).to_invertedcase()
    }
    fn to_altcase(&self, start_with: Case) -> String {
        String::from_str(self).to_altcase(start_with)
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
    assert_eq!(String::from_str("SOMETHING").to_altcase(Case::Lower), "sOmEtHiNg");
    assert_eq!(String::from_str("SOMETHING").to_altcase(Case::Upper), "SoMeThInG");
    assert_eq!(String::from_str("som3thing").to_altcase(Case::Lower), "sOm3ThInG");
    assert_eq!(String::from_str("som3thing").to_altcase(Case::Upper), "SoM3tHiNg");
    assert_eq!(String::from_str("some word").to_altcase(Case::Lower), "sOmE wOrD");
}

#[test]
fn str_lower() {
    assert_eq!("SOMETHING".to_lowercase(), "something");
    assert_eq!("sOmEtHiNg".to_lowercase(), "something");
    assert_eq!("Word WORD2".to_lowercase(), "word word2");
}

#[test]
fn str_capitalize() {
    assert_eq!("SOMETHING".to_capitalized(), "Something");
    assert_eq!("sOmEtHiNg".to_capitalized(), "Something");
    assert_eq!("Word WORD2".to_capitalized(), "Word word2");
}

#[test]
fn str_swapped() {
    assert_eq!("SOMETHING".to_invertedcase(), "something");
    assert_eq!("SoMeThInG".to_invertedcase(), "sOmEtHiNg");
    assert_eq!("Some WorD2".to_invertedcase(), "sOME wORd2");
}

#[test]
fn str_alternating() {
    assert_eq!("SOMETHING".to_altcase(Case::Lower), "sOmEtHiNg");
    assert_eq!("SOMETHING".to_altcase(Case::Upper), "SoMeThInG");
    assert_eq!("som3thing".to_altcase(Case::Lower), "sOm3ThInG");
    assert_eq!("som3thing".to_altcase(Case::Upper), "SoM3tHiNg");
    assert_eq!("some word".to_altcase(Case::Lower), "sOmE wOrD");
}
