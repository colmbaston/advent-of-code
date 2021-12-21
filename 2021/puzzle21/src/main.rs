use std::collections::HashMap;
use std::ops::{ Index, IndexMut };

fn main()
{
    let input = State::parse(include_str!("../input.txt"));

    let mut state = input.clone();
    for roll in (6 ..).step_by(9)
    {
        if state.step((roll % 100) as u8) >= 1000
        {
            println!("{}", state[!state.turn].1 * (roll / 3 + 1));
            break
        }
    }

    let mut one_wins  = 0;
    let mut two_wins  = 0;
    let mut universes = HashMap::new();
    universes.insert(input, 1u64);
    while let Some(state) = universes.keys().cloned().next()
    {
        let count = universes.remove(&state).unwrap();
        for (roll, freq) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)].into_iter()
        {
            let mut state = state.clone();
            let k = if state.step(roll) >= 21
            {
                if state.turn { &mut one_wins } else { &mut two_wins }
            }
            else
            {
                universes.entry(state).or_insert(0)
            };
            *k += count * freq;
        }
    }
    println!("{}", one_wins.max(two_wins));
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State
{
    turn:       bool,
    player_one: (u8, u32),
    player_two: (u8, u32)
}

impl State
{
    fn parse(s : &str) -> State
    {
        let mut i = s.lines().map(|l| l[28 ..].parse().unwrap());

        State
        {
            turn:       false,
            player_one: (i.next().unwrap(), 0),
            player_two: (i.next().unwrap(), 0)
        }
    }

    fn step(&mut self, roll : u8) -> u32
    {
        let active_player = !self.turn;
        self.turn         = active_player;
        let (pos, score)  = &mut self[active_player];

        *pos    = (*pos + roll - 1) % 10 + 1;
        *score +=  *pos as u32;

        *score
    }
}

impl Index<bool> for State
{
    type Output = (u8, u32);

    fn index(&self, i : bool) -> &(u8, u32)
    {
        if i { &self.player_one } else { &self.player_two }
    }
}

impl IndexMut<bool> for State
{
    fn index_mut(&mut self, i : bool) -> &mut (u8, u32)
    {
        if i { &mut self.player_one } else { &mut self.player_two }
    }
}
