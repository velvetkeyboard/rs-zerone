pub trait AsBinary {
    fn as_binary(&self) -> String;
}

impl AsBinary for i32 {
    fn as_binary(&self) -> String {
        let mut quotient: i32 = *self;
        let mut remainder: i32;
        let mut ret: String = String::from("");
        while quotient > 0 {
            remainder = quotient % 2;
            quotient = quotient / 2;
            ret.push_str(&remainder.to_string());
        }
        // Filling up zeroes
        for _d in 1..=(8 - (ret.len() % 8)) {
            ret.push_str("0");
        }
        //println!("{}", 8 - (ret.len()%8));
        return ret.chars().rev().collect::<String>();
    }
}

pub fn to_binary(number: i32) -> String {
    let mut quotient: i32 = number;
    let mut remainder: i32;
    let mut ret: String = String::from("");
    loop {
        remainder = quotient % 2;
        quotient = quotient / 2;
        ret.push_str(&remainder.to_string());
        if quotient <= 0  {
            break;
        }
    }
    // Filling up zeroes
    for _d in 1..=(8 - (ret.len() % 8)) {
        ret.push_str("0");
    }
    return ret.chars().rev().collect::<String>();
}

#[cfg(test)]
#[path = "./tests/binary.rs"]
mod binary_test;
