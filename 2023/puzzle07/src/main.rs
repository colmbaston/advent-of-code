#![feature(slice_group_by, iter_next_chunk)]
use std::cmp::Ordering;

fn main()
{
    let mut hands = include_str!("../input.txt").lines()
                                                .map(|l| (Hand::parse(&l[..5]).unwrap(), l[6..].parse::<u32>().unwrap()))
                                                .collect::<Vec<(Hand, u32)>>();

    hands.sort_unstable();
    println!("{}", hands.iter().zip(1..)
                        .map(|((_, bid), rank)| bid * rank)
                        .sum::<u32>());

    hands.iter_mut().for_each(|(hand, _)| hand.joker_mode());
    hands.sort_unstable();
    println!("{}", hands.iter().zip(1..)
                        .map(|((_, bid), rank)| bid * rank)
                        .sum::<u32>());
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Hand([Card ; 5]);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card { Joker, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace }

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType { HighCard, OnePair, TwoPair, ThreeOfAKind, FullHouse, FourOfAKind, FiveOfAKind }

impl Hand
{
    fn hand_type(&self) -> HandType
    {
        let mut hand = self.0;
        hand.sort_unstable();

        let jokers = match hand.iter().copied().position(|c| c != Card::Joker)
        {
            None    => return HandType::FiveOfAKind,
            Some(p) => p
        };

        let mut counts = hand[jokers..].group_by(|a, b| a == b)
                                       .map(|g| g.len())
                                       .collect::<Vec<usize>>();

        counts.sort_unstable();
        *counts.last_mut().unwrap() += jokers;

        match counts[..]
        {
            [5]             => HandType::FiveOfAKind,
            [1, 4]          => HandType::FourOfAKind,
            [2, 3]          => HandType::FullHouse,
            [1, 1, 3]       => HandType::ThreeOfAKind,
            [1, 2, 2]       => HandType::TwoPair,
            [1, 1, 1, 2]    => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _               => unreachable!()
        }
    }

    fn joker_mode(&mut self)
    {
        for card in self.0.iter_mut()
        {
            if let Card::Jack = card { *card = Card::Joker }
        }
    }

    fn parse(s : &str) -> Option<Hand>
    {
        Some(Hand(s.bytes().map(Card::parse).next_chunk().ok()?))
    }
}

impl PartialOrd for Hand
{
    fn partial_cmp(&self, other : &Hand) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

impl Ord for Hand
{
    fn cmp(&self, other : &Hand) -> Ordering
    {
        self.hand_type()
            .cmp(&other.hand_type())
            .then(self.0.cmp(&other.0))
    }
}

impl Card
{
    fn parse(b : u8) -> Card
    {
        match b
        {
            b'A' => Card::Ace,
            b'2' => Card::Two,
            b'3' => Card::Three,
            b'4' => Card::Four,
            b'5' => Card::Five,
            b'6' => Card::Six,
            b'7' => Card::Seven,
            b'8' => Card::Eight,
            b'9' => Card::Nine,
            b'T' => Card::Ten,
            b'J' => Card::Jack,
            b'Q' => Card::Queen,
            b'K' => Card::King,
            _    => unreachable!()
        }
    }
}
