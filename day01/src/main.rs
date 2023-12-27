// use std::collections::HashMap;
use std::fs;

fn main() {
    let file =
        fs::read_to_string("./src/inputs").expect("Should read  the file and convert to a string");

    let input = file
        .split("\n")
        .filter(|i| !i.is_empty())
        .collect::<Vec<&str>>();

    // let input: Vec<&str> = vec![
    //     "two1nine",
    //     "eightwothree",
    //     "zoneight234",
    //     "7pqrstsixteen",
    //     "fivezg8dsadsadttwoneg",
    //     "zoneight5",
    //     "oneoneight",
    //     "oneoneightone",
    //     "oneoneightoneight",
    //     "1rdtwofjvdllht5eightsixfourbl",
    // ];

    let from_digits = calibration_to_digit(input);
    //  println!("{:#?}", from_digits);
    let sum = calibration(from_digits);
    println!("sum: {}", sum)
}
fn calibration(input: Vec<String>) -> u32 {
    let mut numbers: Vec<u32> = Vec::new();

    for item in input {
        let c: std::str::Chars<'_> = item.chars();
        let digits: Vec<char> = c.filter(|c| c.is_digit(10)).collect();

        let (x, y) = (digits[0], digits[digits.len() - 1]);

        let n = String::from(x.to_string() + &y.to_string());

        match n.parse::<u32>() {
            Ok(n) => numbers.push(n),
            Err(_e) => println!("erro"),
        }
    }

    numbers.iter().sum::<u32>()
}

fn calibration_to_digit(input: Vec<&str>) -> Vec<String> {
    let digits = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let spelled = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut words: Vec<String> = Vec::new();

    // HACK: K = key, V = value. 
    // transforma dois vec's num hashmap (objeto js)

    // let lookup: HashMap<&&str, &&str> = spelled.iter().zip(digits.iter()).collect();
    // println!("{:?}", lookup.get(&"two").unwrap());

    for (i, _) in input.iter().enumerate() {
        let mut index = 0;
        let mut word = String::new();

        for c in input[i].chars() {
            let inp = &input[i][index..];

            for s in 0..spelled.len() {
                if inp.starts_with(spelled[s]) {
                    word = word + spelled[s];
                    word = word.replace(spelled[s], digits[s]);
                }
            }

            if c.is_digit(10) {
                word = word + &c.to_string();
            }

            index += 1;
        }

        words.push(word.to_string());
    }

    words
}
