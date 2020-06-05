//! Functions over generic types.

///Inverts a string.
pub fn invert(s: String) -> String {
    s.chars().rev().collect()
}

///Prompts the user to input a number in a specified range until a valid input is passed.
/// # Arguments
/// * `prompt` - The message the user is prompted to (it should indicate the possible inputs).
/// * `range` - The interval (including the endpoints) of accepted inputs.
pub fn dialogue(prompt: &str, range: (u8, u8)) -> usize {
    use std::io;
    assert!(range.0 < range.1);
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse() {
            Ok(num) => {
                if num as u8 >= range.0 && num as u8 <= range.1 { return num; }
            }
            Err(_) => (),
        }
        println!("Invalid input. Trying again...");
    }
}

/// Plots an array of integers to a histogram.
///  # Arguments
/// * `symbol` - The character(s) with which to print the columns.
/// * `gap` - The space between the columns.
/// # Example
/// ```use generics;
/// println!("{}", generics::hist(&[4,3,6,2,5], "**", 2));
///
///  6           **
///  5           **      **
///  4   **      **      **
///  3   **  **  **      **
///  2   **  **  **  **  **
///  1   **  **  **  **  **
///     0/1   2   3   4   5+
/// ```
pub fn hist(l: &[usize], symbol: &str, gap: usize) -> String {
    fn blanks(n: usize) -> String {
        " ".repeat(n)
    }
    let mut result = String::from("\n");
    for height in 0..*l.iter().max().unwrap() {
        // the hist gets built rotated by 180°, it'll need to be inverted.
        for i in l.iter().rev() {
            if i > &height {
                result += &format!("{}{}", symbol, blanks(gap));
            } else {
                result += &format!("{}{}", blanks(symbol.len()), blanks(gap));
            }
        }
        let mut tmp = format!("\n{:>2} ", height + 1); // y axis
        tmp = invert(tmp); // this substring needs an even number of inversions
        result += &tmp;       // to maintain the left to right orientation of the digits.
    }
    result = invert(result);
    result += &format!("{}{}0/1", blanks(symbol.len()), blanks(gap)); // x axis
    for i in 2..l.len() + 1 {
        result += &format!("{}{}{}", blanks(gap), blanks(symbol.len() - 1), i)
    }
    result + "+\n"
}

///Returns the minimum element of a f64 array.
pub fn min(arr: &[f64]) -> f64 {
    let mut _min = arr[0];
    for el in arr {
        if el < &_min {_min = *el;}
    }
    _min
}

///Returns the arithmetic mean of a f64 array.
pub fn avg(arr: &[f64]) -> f64 {
    arr.iter().sum::<f64>() / arr.len() as f64
}

///Returns the maximum element of a f64 array.
pub fn max(arr: &[f64]) -> f64 {
    let mut _max = arr[0];
    for el in arr {
        if el > &_max {_max = *el;}
    }
    _max
}

#[cfg(test)]
#[test]
fn test() {
    use rand::Rng;
    let mut l = [0; 5];
    let mut k = [0.0; 5];
    for i in 0..5 {
        l[i] = rand::thread_rng().gen_range(1, 10);
        k[i] = l[i] as f64;
    }
    println!("{:?}", l);
    println!("{}", hist(&l, "*", 1));
    println!("min: {} avg: {} max: {}",
             min(&k), avg(&k), max(&k));
    println!("{}", hist(&[4,3,6,2,5], "**", 2));
}