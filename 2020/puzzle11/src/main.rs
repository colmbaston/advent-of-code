fn main()
{
    let mut input = parse_grid(include_str!("../input.txt"));

    let mut current = input.clone();
    loop
    {
        let next = step_one(&current);
        if current == next { break } else { current = next }
    }
    println!("{}", current.iter().map(|r| r.iter()).flatten().filter(|c| **c == Cell::Occupied).count());

    loop
    {
        let next = step_two(&input);
        if input == next { break } else { input = next }
    }
    println!("{}", input.iter().map(|r| r.iter()).flatten().filter(|c| **c == Cell::Occupied).count());
}

#[derive(Clone, PartialEq, Eq)]
enum Cell
{
    Floor,
    Empty,
    Occupied
}

type Grid = Vec<Vec<Cell>>;

fn parse_grid(s : &str) -> Grid
{
    s.lines().map(|l| l.bytes().map(|b| match b
    {
        b'.' => Cell::Floor,
        b'L' => Cell::Empty,
        b'#' => Cell::Occupied,
        _    => unreachable!()
    })
    .collect()).collect()
}

const ADJACENT : [(isize, isize); 8] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

fn step_one(seats : &Grid) -> Grid
{
    let adjacent = |x, y|
    {
        ADJACENT.iter()
                .filter_map(|&(a, b)| (y as isize)
                    .checked_add(b)
                    .and_then(|y| seats.get(y as usize).and_then(|r| (x as isize)
                        .checked_add(a)
                        .and_then(|x| r.get(x as usize)))))
                .filter(|c| **c == Cell::Occupied)
                .count()
    };

    seats.iter().enumerate().map(|(y, r)| r.iter().enumerate().map(|(x, c)| match c
    {
        Cell::Floor    => Cell::Floor,
        Cell::Empty    => if adjacent(x, y) == 0 { Cell::Occupied } else { Cell::Empty    },
        Cell::Occupied => if adjacent(x, y) >= 4 { Cell::Empty    } else { Cell::Occupied }
    })
    .collect()).collect()
}

fn step_two(seats : &Grid) -> Grid
{
    let width   = seats[0].len() as isize;
    let height  = seats.len()    as isize;
    let visible = |x, y|
    {
        ADJACENT.iter()
                .filter_map(|&(a, b)|
                {
                    (1..).map(|k| (x as isize + a*k, y as isize + b*k))
                         .take_while(|&(x, y)| 0 <= x && x < width && 0 <= y && y < height)
                         .find_map(|(x, y)| match &seats[y as usize][x as usize]
                         {
                             Cell::Floor => None,
                             c           => Some(c)
                         })
                })
                .filter(|c| **c == Cell::Occupied)
                .count()
    };

    seats.iter().enumerate().map(|(y, r)| r.iter().enumerate().map(|(x, c)| match c
    {
        Cell::Floor    => Cell::Floor,
        Cell::Empty    => if visible(x, y) == 0 { Cell::Occupied } else { Cell::Empty    },
        Cell::Occupied => if visible(x, y) >= 5 { Cell::Empty    } else { Cell::Occupied }
    })
    .collect()).collect()
}
