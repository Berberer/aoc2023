const NUMBER_WORDS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn search_numbers(search_string: &str, include_number_words: bool) -> Vec<u32> {
    let mut numbers = Vec::new();
    let mut substring = String::new();

    for c in search_string.chars() {
        if c.is_ascii_digit() {
            numbers.push(c.to_digit(10).unwrap());
            substring.clear();
        } else if include_number_words {
            substring.push(c);

            if substring.len() > 2 {
                for (number_word, number) in NUMBER_WORDS {
                    if substring.ends_with(number_word) {
                        numbers.push(number);
                        break;
                    }
                }
            }
        }
    }

    numbers
}

fn get_calibration_number(numbers: Vec<u32>) -> u32 {
    let calibration_number_string = format!("{}{}", numbers[0], numbers[numbers.len() - 1]);
    calibration_number_string.parse::<u32>().unwrap()
}

fn main() {
    let input = include_str!("../inputs/data_day_1.txt");
    let input_elements = parse_input(input);

    // Solution for puzzle 1
    let calibration_numbers_sum = input_elements
        .iter()
        .cloned()
        .map(|s| search_numbers(s, false))
        .map(get_calibration_number)
        .sum::<u32>();
    println!("The sum of all calibration numbers with digits only is {calibration_numbers_sum}");

    // Solution for puzzle 2
    let calibration_numbers_sum = input_elements
        .iter()
        .cloned()
        .map(|s| search_numbers(s, true))
        .map(get_calibration_number)
        .sum::<u32>();
    println!(
        "The sum of all calibration numbers including number words is {calibration_numbers_sum}"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_search() {
        assert_eq!(search_numbers("abc", true), vec![]);
        assert_eq!(search_numbers("1abc2", true), vec![1, 2]);
        assert_eq!(search_numbers("1xyztwo", true), vec![1, 2]);
        assert_eq!(search_numbers("oneabc2", true), vec![1, 2]);
        assert_eq!(
            search_numbers("seven1abc2eight3nine", true),
            vec![7, 1, 2, 8, 3, 9]
        );
    }

    #[test]
    fn test_calibration_number() {
        assert_eq!(get_calibration_number(vec![1, 2, 3]), 13);
    }
}
