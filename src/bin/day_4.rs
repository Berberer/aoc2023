use std::cmp::min;
use std::collections::HashSet;
use std::str::SplitWhitespace;

#[derive(PartialEq, Debug)]
struct ScratchCard {
    id: u32,
    winning_numbers: HashSet<u32>,
    card_numbers: HashSet<u32>,
}

impl ScratchCard {
    fn from_line(line: &str) -> Self {
        let (card_id_str, numbers_str) = line.trim().split_once(':').unwrap();

        let id = card_id_str
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let (winning_numbers_str, card_numbers_str) = numbers_str.trim().split_once('|').unwrap();

        ScratchCard {
            id,
            winning_numbers: parse_numbers(winning_numbers_str.split_whitespace()),
            card_numbers: parse_numbers(card_numbers_str.split_whitespace()),
        }
    }

    fn get_winning_number_of_card(&self) -> HashSet<u32> {
        self.winning_numbers
            .intersection(&self.card_numbers)
            .cloned()
            .collect()
    }

    fn get_score_of_card(&self) -> u32 {
        let winning_number_of_card = self.get_winning_number_of_card();

        if winning_number_of_card.is_empty() {
            0
        } else {
            2u32.pow((winning_number_of_card.len() as u32) - 1)
        }
    }
}

fn parse_input(input: &str) -> Vec<ScratchCard> {
    input.lines().map(ScratchCard::from_line).collect()
}

fn parse_numbers(number_parts: SplitWhitespace) -> HashSet<u32> {
    number_parts.map(|s| s.parse::<u32>().unwrap()).collect()
}

fn main() {
    let input = include_str!("../inputs/data_day_4.txt");
    let scratch_cards = parse_input(input);

    // Solution for puzzle 1
    let sum_of_scratch_card_points = scratch_cards
        .iter()
        .map(ScratchCard::get_score_of_card)
        .sum::<u32>();
    println!("The sum of the scratch card points is {sum_of_scratch_card_points}");

    // Solution for puzzle 2
    let mut copy_counter = vec![1u32; scratch_cards.len()];
    for (i, scratch_card) in scratch_cards.iter().enumerate() {
        let winning_numbers_count = scratch_card.get_winning_number_of_card().len();
        if winning_numbers_count > 0 && i < (copy_counter.len() - 1) {
            let end_counter = min(copy_counter.len(), i + winning_numbers_count);
            for n in (i + 1)..=end_counter {
                copy_counter[n] += copy_counter[i];
            }
        }
    }
    let number_of_copies = copy_counter.iter().sum::<u32>();
    println!("The number of scratch card copies is {number_of_copies}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parsing() {
        let scratch_card = ScratchCard::from_line("Game 123: 1 2 3 | 11 12 13");
        assert_eq!(
            scratch_card,
            ScratchCard {
                id: 123,
                winning_numbers: HashSet::from([1, 2, 3]),
                card_numbers: HashSet::from([11, 12, 13]),
            }
        );
    }

    #[test]
    fn test_winning_numbers_of_card() {
        let scratch_card = ScratchCard {
            id: 1,
            winning_numbers: HashSet::from([1, 2, 3]),
            card_numbers: HashSet::from([1, 3]),
        };
        assert_eq!(
            scratch_card.get_winning_number_of_card(),
            HashSet::from([1, 3])
        );
    }

    #[test]
    fn test_score_of_card() {
        let scratch_card = ScratchCard {
            id: 1,
            winning_numbers: HashSet::from([1, 2, 3]),
            card_numbers: HashSet::from([4]),
        };
        assert_eq!(scratch_card.get_score_of_card(), 0);

        let scratch_card = ScratchCard {
            id: 1,
            winning_numbers: HashSet::from([1, 2, 3]),
            card_numbers: HashSet::from([1]),
        };
        assert_eq!(scratch_card.get_score_of_card(), 1);

        let scratch_card = ScratchCard {
            id: 1,
            winning_numbers: HashSet::from([1, 2, 3]),
            card_numbers: HashSet::from([1, 2, 3]),
        };
        assert_eq!(scratch_card.get_score_of_card(), 4);
    }
}
