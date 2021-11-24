use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main()
{
    let mut input = include_str!("../input.txt").lines().map(|s| s.split(": ").nth(1).unwrap().parse::<i32>().unwrap());
    let boss_hp   = input.next().unwrap();
    let boss_d    = input.next().unwrap();
    let mut queue = BinaryHeap::new();

    for &hard in [false, true].iter()
    {
        queue.clear();
        queue.push((Reverse(0), State
        {
            boss_hp,
            boss_d,
            turn:     false,
            hp:       50,
            mana:     500,
            shield:   0,
            poison:   0,
            recharge: 0
        }));

        while let Some((Reverse(m), mut state)) = queue.pop()
        {
            state.turn = !state.turn;

            if state.turn && hard
            {
                state.hp -= 1;
            }

            if state.hp <= 0
            {
                continue
            }

            let armour = if state.shield   > 0 { state.shield   -= 1;                    7 } else { 0 };
                         if state.poison   > 0 { state.poison   -= 1; state.boss_hp -=   3 }
                         if state.recharge > 0 { state.recharge -= 1; state.mana    += 101 }

            if state.boss_hp <= 0
            {
                println!("{}", m);
                break
            }

            if state.turn
            {
                if state.mana >= 53
                {
                    let mut state = state.clone();
                    state.mana    -= 53;
                    state.boss_hp -=  4;
                    queue.push((Reverse(m+53), state));
                }

                if state.mana >= 73
                {
                    let mut state = state.clone();
                    state.mana    -= 73;
                    state.boss_hp -=  2;
                    state.hp      +=  2;
                    queue.push((Reverse(m+73), state));
                }

                if state.mana >= 113 && state.shield == 0
                {
                    let mut state = state.clone();
                    state.mana  -= 113;
                    state.shield =   6;
                    queue.push((Reverse(m+113), state));
                }

                if state.mana >= 173 && state.poison == 0
                {
                    let mut state = state.clone();
                    state.mana  -= 173;
                    state.poison =   6;
                    queue.push((Reverse(m+173), state));
                }

                if state.mana >= 229 && state.recharge == 0
                {
                    state.mana    -= 229;
                    state.recharge =   5;
                    queue.push((Reverse(m+229), state));
                }
            }
            else
            {
                state.hp -= state.boss_d - armour;
                queue.push((Reverse(m), state.clone()))
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct State
{
    boss_hp  : i32,
    boss_d   : i32,
    turn     : bool,
    hp       : i32,
    mana     : i32,
    shield   : i32,
    poison   : i32,
    recharge : i32
}
