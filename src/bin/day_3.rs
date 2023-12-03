use std::collections::HashMap;
use std::ops::RangeInclusive;

#[derive(PartialEq, Eq, Clone, Debug)]
struct PartNumber {
    number: u32,
    x_coord_range: RangeInclusive<usize>,
    y_coord: usize,
}

impl PartNumber {
    fn get_neighboring_area(&self) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
        let neighboring_x_coordinate_range = if *self.x_coord_range.start() == 0 {
            0..=(self.x_coord_range.end() + 1)
        } else {
            (self.x_coord_range.start() - 1)..=(self.x_coord_range.end() + 1)
        };

        let neighboring_y_coordinate_range = if self.y_coord == 0 {
            0..=(self.y_coord + 1)
        } else {
            (self.y_coord - 1)..=(self.y_coord + 1)
        };

        (
            neighboring_x_coordinate_range,
            neighboring_y_coordinate_range,
        )
    }
}

fn parse_input(input: &str) -> (HashMap<(usize, usize), char>, Vec<PartNumber>) {
    let mut symbol_positions = HashMap::new();
    let mut part_number_candidates = Vec::new();

    for (y_coordinate, line) in input.lines().enumerate() {
        let mut current_part_number: Option<PartNumber> = None;

        for (x_coordinate, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                let digit = c.to_digit(10).unwrap();

                if let Some(part_number) = current_part_number.as_mut() {
                    part_number.number = part_number.number * 10 + digit;
                    part_number.x_coord_range = *part_number.x_coord_range.start()..=x_coordinate;
                } else {
                    current_part_number = Some(PartNumber {
                        number: digit,
                        y_coord: y_coordinate,
                        x_coord_range: x_coordinate..=x_coordinate,
                    });
                }
            } else {
                if let Some(part_number) = current_part_number {
                    part_number_candidates.push(part_number);
                    current_part_number = None;
                }

                if c != '.' {
                    symbol_positions.insert((x_coordinate, y_coordinate), c);
                }
            }
        }

        if let Some(part_number) = current_part_number {
            part_number_candidates.push(part_number);
        }
    }

    (symbol_positions, part_number_candidates)
}

fn is_valid_part_number(
    part_number_candidate: &PartNumber,
    symbol_positions: &HashMap<(usize, usize), char>,
) -> bool {
    let (valid_symbol_x_coordinate_range, valid_symbol_y_coordinate_range) =
        part_number_candidate.get_neighboring_area();

    for x_coordinate in valid_symbol_x_coordinate_range {
        for y_coordinate in valid_symbol_y_coordinate_range.clone() {
            if symbol_positions.contains_key(&(x_coordinate, y_coordinate)) {
                return true;
            }
        }
    }

    false
}

fn get_neighboring_part_numbers<'a>(
    coordinates: &(usize, usize),
    part_numbers: &'a [PartNumber],
) -> Vec<&'a PartNumber> {
    part_numbers
        .iter()
        .filter(|part_numbers| {
            let (neighboring_x_coordinate_range, neighboring_y_coordinate_range) =
                part_numbers.get_neighboring_area();

            neighboring_x_coordinate_range.contains(&coordinates.0)
                && neighboring_y_coordinate_range.contains(&coordinates.1)
        })
        .collect()
}

fn main() {
    let input = include_str!("../inputs/data_day_3.txt");
    let (symbol_positions, part_number_candidates) = parse_input(input);
    let valid_part_numbers = part_number_candidates
        .iter()
        .filter(|part_number_candidate| {
            is_valid_part_number(part_number_candidate, &symbol_positions)
        })
        .cloned()
        .collect::<Vec<PartNumber>>();

    // Solution for puzzle 1
    let sum_of_part_numbers = valid_part_numbers
        .iter()
        .map(|part_number| part_number.number)
        .sum::<u32>();
    println!("The sum of the valid part numbers is {sum_of_part_numbers}");

    // Solution for puzzle
    let gear_symbol_positions = symbol_positions
        .iter()
        .filter(|(_, c)| **c == '*')
        .map(|(coords, c)| (*coords, *c))
        .collect::<HashMap<(usize, usize), char>>();

    let mut sum_of_gear_ratios = 0;
    for gear_coordinates in gear_symbol_positions.keys() {
        let neighboring_part_numbers =
            get_neighboring_part_numbers(gear_coordinates, &valid_part_numbers);
        if neighboring_part_numbers.len() == 2 {
            sum_of_gear_ratios +=
                neighboring_part_numbers[0].number * neighboring_part_numbers[1].number;
        }
    }
    println!("The sum of gear ratios is {sum_of_gear_ratios}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parsing() {
        let (symbol_positions, part_number_candidates) = parse_input("..1234#567\n8....9...+");
        assert_eq!(
            symbol_positions,
            HashMap::from([((6, 0), '#'), ((9, 1), '+')])
        );
        assert_eq!(
            part_number_candidates,
            vec![
                PartNumber {
                    number: 1234,
                    x_coord_range: 2..=5,
                    y_coord: 0,
                },
                PartNumber {
                    number: 567,
                    x_coord_range: 7..=9,
                    y_coord: 0,
                },
                PartNumber {
                    number: 8,
                    x_coord_range: 0..=0,
                    y_coord: 1,
                },
                PartNumber {
                    number: 9,
                    x_coord_range: 5..=5,
                    y_coord: 1,
                },
            ]
        );
    }

    #[test]
    fn test_get_part_number_neighboring_area() {
        let part_number = PartNumber {
            number: 1,
            x_coord_range: 0..=3,
            y_coord: 5,
        };
        let (neighboring_x_coordinate_range, neighboring_y_coordinate_range) =
            part_number.get_neighboring_area();

        assert_eq!(neighboring_x_coordinate_range, 0..=4);
        assert_eq!(neighboring_y_coordinate_range, 4..=6);
    }

    #[test]
    fn test_valid_part_number_check() {
        let symbol_positions = HashMap::from([((6, 0), '#'), ((9, 1), '+')]);

        let part_number = PartNumber {
            number: 1,
            x_coord_range: 0..=3,
            y_coord: 1,
        };
        assert!(!is_valid_part_number(&part_number, &symbol_positions));

        let part_number = PartNumber {
            number: 1,
            x_coord_range: 10..=10,
            y_coord: 0,
        };
        assert!(is_valid_part_number(&part_number, &symbol_positions));
    }

    #[test]
    fn test_get_neighboring_part_numbers() {
        let part_numbers = vec![
            PartNumber {
                number: 1,
                x_coord_range: 0..=0,
                y_coord: 0,
            },
            PartNumber {
                number: 2,
                x_coord_range: 5..=5,
                y_coord: 1,
            },
            PartNumber {
                number: 3,
                x_coord_range: 3..=5,
                y_coord: 2,
            },
        ];

        assert_eq!(
            get_neighboring_part_numbers(&(4, 1), &part_numbers),
            vec![
                &PartNumber {
                    number: 2,
                    x_coord_range: 5..=5,
                    y_coord: 1,
                },
                &PartNumber {
                    number: 3,
                    x_coord_range: 3..=5,
                    y_coord: 2,
                },
            ]
        )
    }
}
