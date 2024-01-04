use std::collections::VecDeque;

fn main()
{
    let (players, marbles) = parse(include_str!("../input.txt"));

    println!("{}", play_game(players, marbles));
    println!("{}", play_game(players, marbles * 100));
}

fn parse(s : &str) -> (usize, usize)
{
    fn span_digits(s : &str) -> (&str, &str)
    {
        s.split_at(s.find(|c : char| !c.is_ascii_digit()).unwrap_or(s.len()))
    }

    let (players, s) = span_digits(&s[0..]);
    let (marbles, _) = span_digits(&s[31..]);

    (players.parse().unwrap(), marbles.parse().unwrap())
}

fn play_game(players : usize, marbles : usize) -> usize
{
    let mut scores = vec![0 ; players];
    let mut circle = VecDeque::with_capacity(marbles);

    // need to push the first two in advance so circle.rotate_left(2) doesn't panic
    circle.push_front(0);
    circle.push_front(1);

    for (m, p) in (2 .. marbles).zip((0 .. players).cycle().skip(2))
    {
        if m % 23 != 0
        {
            circle.rotate_left(2);
            circle.push_front(m);
        }
        else
        {
            circle.rotate_right(7);
            scores[p] += m + circle.pop_front().unwrap();
        }
    }

    scores.into_iter().max().unwrap()
}
