use std::rt::io;
use std::rt::io::Reader;

fn main() {
    println("Pick an implementation:");
    println("1: Range Inclusive");
    println("2: Iterator Map");
    println("3: Vector from_fn()");

    let selection = readline();

    println("-----------------");

    match selection {
        ~"1"       => fizz_buzz_v1(),
        ~"2"       => fizz_buzz_v2(),
        ~"3"       => fizz_buzz_v3(),
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

/* Iterator on an inclusive range, printing as it goes */
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

/* Creates an inclusive range of numbers, then maps them to the display strings using get_fizz_buzz().
 * Iterates through the mapped iterator and prints
 */
fn fizz_buzz_v2() -> () {
    for x in std::iter::range_inclusive(1, 100).map(get_fizz_buzz) {
        println!("{}", x);
    }
}

/* Given an integer, returns the correct FizzBuzz output string */
fn get_fizz_buzz (i: int) -> ~str {
    let mod3 = i % 3;
    let mod5 = i % 5;

    if (mod3 == 0 && mod5 == 0) {
        return ~"FizzBuzz";
    }

    if (mod3 == 0) {
        return ~"Fizz";
    }

    if (mod5 == 0) {
        return ~"Buzz";
    }
    
    return i.to_str();
}


/* Uses vec::from_fn to build a vector by calling get_fizz_buzz() 100 times */
fn fizz_buzz_v3() -> () {
    let fizzbuzz: ~[~str] = std::vec::from_fn(100, |i| get_fizz_buzz(1 + i as int));

    for x in fizzbuzz.move_iter() {
        println!("{}", x);
    }
}
