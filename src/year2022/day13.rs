use std::cmp::Ordering;

use nom::character::complete::{char, digit1};
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::{branch::alt, sequence::delimited, IResult};

#[derive(Debug, Clone, Eq, PartialEq)]
enum PacketElement {
    Int(u32),
    List(Vec<PacketElement>),
}

impl Ord for PacketElement {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            PacketElement::Int(left) => match other {
                PacketElement::Int(right) => left.cmp(right),
                PacketElement::List(right) => {
                    let tmp = vec![self.clone(); 1];
                    tmp.cmp(right)
                }
            },
            PacketElement::List(left) => match other {
                PacketElement::Int(_) => {
                    let tmp = vec![other.clone(); 1];
                    left.cmp(&tmp)
                }
                PacketElement::List(right) => left.cmp(right),
            },
        }
    }
}

impl PartialOrd for PacketElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PacketElement {
    fn parse(input: &str) -> IResult<&str, PacketElement> {
        alt((PacketElement::parse_list, PacketElement::parse_int))(input)
    }

    fn parse_list(input: &str) -> IResult<&str, PacketElement> {
        map(
            delimited(
                char('['),
                separated_list0(char(','), PacketElement::parse),
                char(']'),
            ),
            PacketElement::List,
        )(input)
    }

    fn parse_int(input: &str) -> IResult<&str, PacketElement> {
        map(digit1, |s: &str| {
            PacketElement::Int(s.parse::<u32>().unwrap())
        })(input)
    }
}

pub fn part1(input: &str) -> String {
    // Input
    let pairs = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|lines| {
            [
                PacketElement::parse(lines[0]).unwrap().1,
                PacketElement::parse(lines[1]).unwrap().1,
            ]
        })
        .collect::<Vec<_>>();

    // Star 1
    let sum1: usize = pairs
        .iter()
        .enumerate()
        .filter_map(|(index, pair)| {
            if pair[0] < pair[1] {
                Some(index + 1)
            } else {
                None
            }
        })
        .sum();
    
    sum1.to_string()
}


pub fn part2(input: &str) -> String {
     // Input
     let pairs = input
     .lines()
     .collect::<Vec<_>>()
     .chunks(3)
     .map(|lines| {
         [
             PacketElement::parse(lines[0]).unwrap().1,
             PacketElement::parse(lines[1]).unwrap().1,
         ]
     })
     .collect::<Vec<_>>();

    // Star 2
    let mut all = pairs.into_iter().flatten().collect::<Vec<_>>();
    let divider2 = PacketElement::List(vec![PacketElement::List(vec![PacketElement::Int(2)])]);
    let divider6 = PacketElement::List(vec![PacketElement::List(vec![PacketElement::Int(6)])]);
    all.push(divider2.clone());
    all.push(divider6.clone());
    all.sort();

    let index2 = all.iter().position(|elem| *elem == divider2).unwrap() + 1;
    let index6 = all.iter().position(|elem| *elem == divider6).unwrap() + 1;
    let prod2 = index2 * index6;

    prod2.to_string()
}

crate::run!();

crate::test_example_aoc!(13, 140);

crate::test_aoc!(5580, 26200);
