use std::collections::{BTreeMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{
        self, digit1, line_ending, space0, space1,
    },
    multi::{fold_many1, separated_list1},
    sequence::{
        delimited, separated_pair, terminated, tuple,
    },
    IResult, Parser,
};

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

impl Card {
    fn score(&self) -> u32 {
        match self.num_matches().checked_sub(1) {
            Some(num) => 2u32.pow(num as u32),
            None => 0,
        }
    }

    fn num_matches(&self) -> usize {
        self.winning_numbers
            .intersection(&self.my_numbers)
            .count()
    }
}

fn set(input: &str) -> IResult<&str, HashSet<u32>> {
    fold_many1(
        terminated(complete::u32, space0),
        HashSet::new,
        |mut acc: HashSet<_>, item| {
            acc.insert(item);
            acc
        },
    )(input)
}

fn card(input: &str) -> IResult<&str, Card> {
    let (input, _) = delimited(
        tuple((tag("Card"), space1)),
        digit1,
        tuple((tag(":"), space1)),
    )(input)?;
    separated_pair(set, tuple((tag("|"), space1)), set)
        .map(|(winning_numbers, my_numbers)| Card {
            winning_numbers,
            my_numbers,
        })
        .parse(input)
}

fn cards(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, card)(input)
}

fn process(input: &str) -> String {
    let (_, card_data) =
        cards(input).expect("a valid parse");

    let data = card_data
        .iter()
        .map(|card| card.num_matches())
        .collect::<Vec<_>>();
    let store = (0..card_data.len())
        .map(|index| (index, 1))
        .collect::<BTreeMap<usize, u32>>();
    let result = data
        .iter()
        .enumerate()
        .fold(store, |mut acc, (index, card_score)| {
            for i in (index + 1)
                ..(index + 1 + *card_score as usize)
            {
                let to_add = *acc.get(&index).unwrap();
                acc.entry(i)
                    .and_modify(|value| *value += to_add);
            }
            acc
        })
        .values()
        .sum::<u32>();

    // result.to_string()
    dbg!(result);
    result.to_string()
}

fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("30", process(input))
    }
}
