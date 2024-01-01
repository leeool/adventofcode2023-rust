use std::fs;

fn main() {
    let input = fs::read_to_string("./src/input").expect("should read the file");

    let rows_string: Vec<&str> = input.split("\n").filter(|r| !r.is_empty()).collect();
    let rows: Vec<Vec<&str>> = rows_string
        .iter()
        .map(|r| r.split("").filter(|r| !r.is_empty()).collect())
        .collect();

    rows.iter().for_each(|r| println!("{:?}", r));
    let empty_vec = rows.iter().map(|_| "").collect::<Vec<&str>>();

    for (n1, r) in rows.iter().enumerate() {
        let mut numbers: Vec<char> = Vec::new();
        for (n2, c) in r.iter().enumerate() {
            // rows[n1 = y][n2 = x]
            let (sub_index_y, _) = n1.overflowing_sub(1);
            let add_index_y = n1 + 1;

            let (sub_index_x, _) = n2.overflowing_sub(1);
            let add_index_x = n2 + 1;

            let target = rows[n1][n2].parse::<char>().unwrap();
            let target_row = rows
                .get(n1)
                .unwrap()
                .iter()
                .map(|t| t.parse::<char>().unwrap())
                .collect::<Vec<char>>();
            let prev_row = rows.get(sub_index_y).unwrap_or(&empty_vec);
            let next_row = rows.get(add_index_y).unwrap_or(&empty_vec);

            let mut prev_col = r.get(sub_index_x).unwrap_or(&"");
            let mut next_col = r.get(add_index_x).unwrap_or(&"");

            let (top, bottom) = (
                prev_row.get(n2).unwrap().parse::<char>().unwrap_or(' '),
                next_row.get(n2).unwrap().parse::<char>().unwrap_or(' '),
            );
            let (left, right) = (
                prev_col.parse::<char>().unwrap_or(' '),
                next_col.parse::<char>().unwrap_or(' '),
            );

            let has_adjacent_chars = has_adjacent_chars(top, bottom, left, right);

            if is_digit(target) && has_adjacent_chars {
                let next_char = &right;
                let prev_char = &left;

                // get_adjacent_chars(&mut numbers, &target_row, n2)

                let a = target_row
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join("");

                let b = a
                    .split(".")
                    .filter(|c| !c.is_empty())
                    .collect::<Vec<&str>>();

                println!("{:?}", b);

                // numbers.push(target.to_digit(10).unwrap());
                // if is_digit(*next_char) {
                //     numbers.push(next_char.to_digit(10).unwrap());
                // } else if is_digit(*prev_char) {
                //     numbers.push(prev_char.to_digit(10).unwrap());
                // } else {
                // }
            }

            println!(
                "{}º -> target: '{}' top: '{}' bottom: '{}' left: '{}' right: '{}'",
                n1, target, top, bottom, left, right
            );
        }
        println!("{:?}", numbers);
        println!("############");

        // println!("{:?}", new_row);
        // let col = &cols[n];
        // let row = &cols[n];
    }

    // println!("{:?}", cols);
}

fn has_adjacent_chars(top: char, bottom: char, left: char, right: char) -> bool {
    let target_chars: Vec<char> = vec!['/', '*', '#', '+', '-', '&', '&', '=', '%', '!', '@'];

    target_chars.contains(&top)
        || target_chars.contains(&bottom)
        || target_chars.contains(&left)
        || target_chars.contains(&right)
}

fn is_digit(target: char) -> bool {
    target.is_digit(10)
}

fn get_adjacent_chars(numbers: &mut Vec<char>, target_row: &Vec<char>, target_index: usize) {
    let target = target_row.get(target_index).unwrap_or(&'.');
    let (sub, sub_invalid) = target_index.overflowing_sub(1);
    let (add, add_invalid) = target_index.overflowing_add(1);
    let target_prev = target_row.get(sub).unwrap_or(&'.');
    let target_next = target_row.get(add).unwrap_or(&'.');
    let mut index_start = 0;

    if !(target.is_digit(10)) {
        return;
    }

    //numbers.push(*target);

    //get_adjacent_chars(numbers, target_row, sub);
    // if target_prev.is_digit(10) {
    //     get_adjacent_chars(numbers, target_row, add);
    //     numbers.push(*target_prev);
    // } else {
    //     get_adjacent_chars(numbers, target_row, sub);
    //     numbers.push(*target_next);
    // }

    // println!("{}", sub);
    // println!("{}", add);
}

fn find_start_index(target_row: &Vec<char>, target_index: usize) {}
