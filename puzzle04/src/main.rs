mod non_descending;
use crate::non_descending::{ NonDescending, DIGITS };

fn main()
{
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut i = input.trim_end().split('-');
    let mut parse_next = || i.next().unwrap().parse::<u64>().unwrap();
    let mut lower = parse_next();
    let mut upper = parse_next();

    let mut lower_array = [0 ; DIGITS];
    let mut upper_array = [0 ; DIGITS];

    for i in (0 .. DIGITS).rev()
    {
        lower_array[i] = (lower % 10) as u8;
        lower /= 10;

        upper_array[i] = (upper % 10) as u8;
        upper /= 10;
    }

    let range = (NonDescending { digits: lower_array }).take_while(|x| *x <= upper_array);
    let (x,y) = range.fold((0, 0), |(x, y), n| { let (a, b) = streaks(&n); (if a {x+1} else {x},
                                                                            if b {y+1} else {y}) });

    println!("{}\n{}", x, y);
}

fn streaks(n : &[u8]) -> (bool, bool)
{
    let mut streak     = 1;
    let mut two_streak = false;

    for i in 0 .. n.len()-1
    {
        if n[i] == n[i+1]
        {
            streak += 1;
            two_streak = true
        }
        else
        {
            if streak == 2
            {
                return (true, true)
            }
            streak = 1;
        }
    }

    (two_streak, streak == 2)
}
