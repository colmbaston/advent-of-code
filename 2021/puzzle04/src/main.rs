fn main()
{
    let (rng, mut boards) = parse_input(include_str!("../input.txt"));

    let mut last_score = None;
    let mut incomplete = Vec::with_capacity(boards.len());
    for &n in rng.iter()
    {
        for mut b in boards.drain(..)
        {
            match mark(n, &mut b)
            {
                None        => incomplete.push(b),
                Some(score) =>
                {
                    if last_score.is_none() { println!("{}", score) }
                    last_score = Some(score);
                }
            }
        }
        std::mem::swap(&mut boards, &mut incomplete);
    }
    println!("{}", last_score.unwrap());
}

type Board = Vec<(u32, bool)>;

fn parse_input(s : &str) -> (Vec<u32>, Vec<Board>)
{
    let mut i  = s.split("\n\n");
    let rng    = i.next().unwrap().split(',').map(|k| k.parse().unwrap()).collect();
    let boards = i.map(|t| t.split_whitespace().map(|k| (k.parse().unwrap(), false)).collect()).collect();

    (rng, boards)
}

fn mark(n : u32, board : &mut Board) -> Option<u32>
{
    for (i, (k, m)) in board.iter_mut().enumerate()
    {
        if !*m && n == *k
        {
            *m = true;

            let row = i / 5;
            let col = i % 5;
            let won = (0 .. 5).map(|o| 5*row + o).all(|j| board[j].1)
                   || (0 .. 5).map(|o| 5*o + col).all(|j| board[j].1);

            return won.then(|| n * board.iter().filter_map(|(k, m)| (!m).then_some(k)).sum::<u32>())
        }
    }

    None
}
