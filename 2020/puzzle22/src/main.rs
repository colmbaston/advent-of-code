use std::hash::{ Hash, Hasher };
use std::collections::{ VecDeque, HashSet, hash_map::DefaultHasher };

fn main()
{
    let (player_one, player_two) = parse_decks(include_str!("../input.txt"));
    println!("{}", score(&combat(player_one.clone(), player_two.clone())));
    println!("{}", score(&recursive_combat(player_one, player_two).0));
}

fn parse_decks(s : &str) -> (VecDeque<u32>, VecDeque<u32>)
{
    let mut it = s.split("\n\n");

    (it.next().unwrap().lines().skip(1).map(|x| x.parse().unwrap()).collect(),
     it.next().unwrap().lines().skip(1).map(|x| x.parse().unwrap()).collect())
}

fn score(deck : &VecDeque<u32>) -> u32
{
    deck.iter().cloned().rev().zip(1..).fold(0, |a, (c, i)| a + c * i)
}

fn combat(mut player_one : VecDeque<u32>, mut player_two : VecDeque<u32>) -> VecDeque<u32>
{
    while !player_two.is_empty()
    {
        let mut x = player_one.pop_front().unwrap();
        let mut y = player_two.pop_front().unwrap();
        if y > x
        {
            std::mem::swap(&mut player_one, &mut player_two);
            std::mem::swap(&mut x, &mut y);
        }
        player_one.push_back(x);
        player_one.push_back(y);
    }
    player_one
}

fn recursive_combat(mut player_one : VecDeque<u32>, mut player_two : VecDeque<u32>) -> (VecDeque<u32>, bool)
{
    let mut visited = HashSet::new();
    loop
    {
        if player_one.is_empty() { break (player_two, false) }
        if player_two.is_empty() { break (player_one, true)  }

        let mut hasher = DefaultHasher::new();
        player_one.hash(&mut hasher);
        player_two.hash(&mut hasher);
        if !visited.insert(hasher.finish()) { break (player_one, true) }

        let mut x = player_one.pop_front().unwrap();
        let mut y = player_two.pop_front().unwrap();

        let winner = if player_one.len() as u32 >= x && player_two.len() as u32 >= y
        {
            recursive_combat(player_one.iter().cloned().take(x as usize).collect(),
                             player_two.iter().cloned().take(y as usize).collect()).1
        }
        else
        {
            x > y
        };

        if !winner { std::mem::swap(&mut x, &mut y) }
        let deck = if winner { &mut player_one } else { &mut player_two };
        deck.push_back(x);
        deck.push_back(y);
    }
}
