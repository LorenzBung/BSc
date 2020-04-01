use std::fmt;
#[derive(Debug)]
pub struct Roman(String);

impl fmt::Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<Roman> for &'static str {
    fn eq(&self, roman: &Roman) -> bool {
        self.to_string() == roman.0
    }
}

impl From<i32> for Roman {
    fn from(arabic_number: i32) -> Self {
        let stages = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let mut an = arabic_number;
        let mut value = String::new();
        for s in stages {
            while an >= s {
                value.push_str(&Roman::to_roman(s));
                an -= s;
            }
        }
        Roman::new(value)
    }
}

impl Roman {
    fn to_roman(number: i32) -> String {
        match number {
            1 => "I",
            4 => "IV",
            5 => "V",
            9 => "IX",
            10 => "X",
            40 => "XL",
            50 => "L",
            90 => "XC",
            100 => "C",
            400 => "CD",
            500 => "D",
            900 => "CM",
            1000 => "M",
            _ => "",
        }
        .to_string()
    }

    fn new(string: String) -> Self {
        Roman(string)
    }
}
