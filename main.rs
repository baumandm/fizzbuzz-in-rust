use std::rt::io;
use std::rt::io::Reader;

fn main() {
    println("Pick an implementation:");
    println("1: Range Inclusive");

    let selection = readline();

    match selection {
        ~"1"       => fizz_buzz_v1(),
        _         => println!("Not a valid option!")
    }

}

fn readline() -> ~str {
    let mut chars: ~[char] = ~[];
    for i in io::stdin().bytes() {
        let ic = i as char;
        if (ic == '\n') {
            break;
        }

        chars.push(ic);
    }

    return std::str::from_chars(chars);
}

fn fizz_buzz_v1() -> () {
    for i in std::iter::range_inclusive(1, 100) {
        let mut printed = false;
        if (i % 3 == 0) {
            print!("Fizz");
            printed = true;
        }
        if (i % 5 == 0) {
            print!("Buzz");
            printed = true
        }

        if (!printed) {
            print!("{}", i);
        }

        println("");
    }
}