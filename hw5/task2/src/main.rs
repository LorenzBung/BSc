use std::fmt;

struct Roman(String);

impl fmt::Display for Roman {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<i32> for Roman {
    fn from(arabic_number: i32) -> Self {
        let stages = vec![1000, 500, 100, 50, 10, 7, 6, 5, 4, 3, 2, 1];
        let mut an = arabic_number;
        let mut value = String::new();
        for s in stages {
            while an >= s{
                value.push_str(&Roman::to_roman(s));
                an -= s;
            }
        }
        Roman::new(value)
    }
}

impl Roman {
    fn to_roman(number:i32) -> String {
        match number {
            1 => "I",
            2 => "II",
            3 => "III",
            4 => "IV",
            5 => "V",
            6 => "VI",
            7 => "VII",
            10 => "X",
            50 => "L",
            100 => "C",
            500 => "D",
            1000 => "M",
            _ => "",
        }.to_string()
    }

    fn new(string: String) -> Self {
        Roman(string)
    }
}


fn main() {
    println!("Hello, world!");
    println!("{} into {}", 2012, Roman::from(2012));
}
