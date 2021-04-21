/// Write a program that given a number (with arbitrary number of digits), converts it into LCD
/// style numbers using the following format:
///     _  _     _  _  _  _  _
///  |  _| _||_||_ |_   ||_||_|
///  | |_  _|  | _||_|  ||_| _|
/// (each digit is 3 lines high)

use std::error::Error;

#[derive(Debug)]
struct Number<'a> {
    digit: u32,
    representation: [&'a str; 3],
}

impl<'a> Number<'a> {
    fn new(digit: u32) -> Self {
        let mut representation = [""; 3];
        match digit {
            0 => {
                representation[0] = " _ ";
                representation[1] = "| |";
                representation[2] = "|_|";
            }
            1 => {
                representation[0] = "   ";
                representation[1] = " | ";
                representation[2] = " | ";
            }
            2 => {
                representation[0] = " _ ";
                representation[1] = " _|";
                representation[2] = "|_ ";
            }
            3 => {
                representation[0] = " _ ";
                representation[1] = " _|";
                representation[2] = " _|";
            }
            4 => {
                representation[0] = "   ";
                representation[1] = "|_|";
                representation[2] = "  |";
            }
            5 => {
                representation[0] = " _ ";
                representation[1] = "|_ ";
                representation[2] = " _|";
            }
            6 => {
                representation[0] = " _ ";
                representation[1] = "|_ ";
                representation[2] = "|_|";
            }
            7 => {
                representation[0] = " _ ";
                representation[1] = "  |";
                representation[2] = "  |";
            }
            8 => {
                representation[0] = " _ ";
                representation[1] = "|_|";
                representation[2] = "|_|";
            }
            9 => {
                representation[0] = " _ ";
                representation[1] = "|_|";
                representation[2] = " _|";
            }
            _ => panic!("error")
        }
        Self{digit, representation}
    }
}

fn banner(digits: u32) {
    let mut numbers: Vec<Number> = Vec::new();
    for digit in digits.to_string().chars().map(|d| d.to_digit(10).unwrap()) {
        numbers.push(Number::new(digit));
    }
    let mut banner0 = String::new();
    let mut banner1 = String::new();
    let mut banner2 = String::new();

    for number in numbers {
        banner0.push_str(number.representation[0]);
        banner1.push_str(number.representation[1]);
        banner2.push_str(number.representation[2]);
    }

    print!("{}\n{}\n{}\n", banner0, banner1, banner2);
}

fn main() -> Result<(), Box<dyn Error>> {

    banner(12310000);

    Ok(())
}
