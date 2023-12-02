use std::cmp::max;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug)]
enum CubeColor {
    Red,
    Blue,
    Green,
}

impl CubeColor {
    fn from_str(string: &str) -> Option<Self> {
        match string.trim().to_lowercase().as_str() {
            "red" => Some(Self::Red),
            "blue" => Some(Self::Blue),
            "green" => Some(Self::Green),
            _ => None,
        }
    }
}

#[derive(PartialEq, Debug)]
struct Game {
    id: u32,
    cube_draws: Vec<Vec<(CubeColor, u32)>>,
}

impl Game {
    fn from_line(line: &str) -> Self {
        let (game_id_str, cube_draws_str) = line.trim().split_once(':').unwrap();

        let id = game_id_str[5..].parse::<u32>().unwrap();

        let mut cube_draws = Vec::new();
        for draw_str in cube_draws_str.trim().split(';') {
            let draw = draw_str
                .split(',')
                .map(|color_amount_str| color_amount_str.trim().split_once(' ').unwrap())
                .map(|(amount_str, color_str)| {
                    (
                        CubeColor::from_str(color_str).unwrap(),
                        amount_str.parse::<u32>().unwrap(),
                    )
                })
                .collect();

            cube_draws.push(draw);
        }

        Game { id, cube_draws }
    }

    fn get_min_cube_counts(&self) -> HashMap<&CubeColor, u32> {
        let mut min_cube_counts = HashMap::new();

        for draw in self.cube_draws.iter() {
            for (cube_color, count) in draw {
                if let Some(min_count) = min_cube_counts.get(cube_color) {
                    min_cube_counts.insert(cube_color, max(*min_count, *count));
                } else {
                    min_cube_counts.insert(cube_color, *count);
                }
            }
        }

        min_cube_counts
    }
}

fn is_game_possible_with_bag(game: &Game, bag_cube_count: &HashMap<&CubeColor, u32>) -> bool {
    let min_cube_count = game.get_min_cube_counts();
    for (cube_color, min_count) in min_cube_count.iter() {
        if let Some(bag_count) = bag_cube_count.get(cube_color) {
            if min_count > bag_count {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

fn main() {
    let input = include_str!("../inputs/data_day_2.txt");
    let games = input.lines().map(Game::from_line).collect::<Vec<Game>>();

    // Solution for puzzle 1
    let bag = HashMap::from([
        (&CubeColor::Blue, 14),
        (&CubeColor::Red, 12),
        (&CubeColor::Green, 13),
    ]);
    let sum_of_possible_game_ids = games
        .iter()
        .filter(|game| is_game_possible_with_bag(game, &bag))
        .map(|game| game.id)
        .sum::<u32>();
    println!("The sum of the IDs of valid games is {sum_of_possible_game_ids}");

    // Solution for puzzle 2
    let sum_of_cube_set_powers = games
        .iter()
        .map(|game| game.get_min_cube_counts().values().product::<u32>())
        .sum::<u32>();
    println!("The sum of the powers of cube sets {sum_of_cube_set_powers}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_from_line() {
        let game = Game::from_line("Game 123: 1 blue, 1 red, 1 green; 999 blue");
        assert_eq!(
            game,
            Game {
                id: 123,
                cube_draws: vec![
                    vec![
                        (CubeColor::Blue, 1),
                        (CubeColor::Red, 1),
                        (CubeColor::Green, 1),
                    ],
                    vec![(CubeColor::Blue, 999)],
                ],
            }
        );
    }

    #[test]
    fn test_min_cube_counts() {
        let game = Game {
            id: 123,
            cube_draws: vec![
                vec![
                    (CubeColor::Blue, 1),
                    (CubeColor::Red, 2),
                    (CubeColor::Green, 3),
                ],
                vec![(CubeColor::Green, 2), (CubeColor::Red, 4)],
                vec![
                    (CubeColor::Blue, 5),
                    (CubeColor::Red, 3),
                    (CubeColor::Green, 1),
                ],
            ],
        };
        assert_eq!(
            game.get_min_cube_counts(),
            HashMap::from([
                (&CubeColor::Blue, 5),
                (&CubeColor::Red, 4),
                (&CubeColor::Green, 3)
            ])
        )
    }

    #[test]
    fn test_is_game_possible_with_bag() {
        let game = Game {
            id: 123,
            cube_draws: vec![vec![
                (CubeColor::Blue, 1),
                (CubeColor::Red, 2),
                (CubeColor::Green, 3),
            ]],
        };

        let bag = HashMap::from([
            (&CubeColor::Blue, 5),
            (&CubeColor::Red, 4),
            (&CubeColor::Green, 3),
        ]);
        assert!(is_game_possible_with_bag(&game, &bag));

        let bag = HashMap::from([
            (&CubeColor::Blue, 5),
            (&CubeColor::Red, 1),
            (&CubeColor::Green, 3),
        ]);
        assert!(!is_game_possible_with_bag(&game, &bag));
    }
}
