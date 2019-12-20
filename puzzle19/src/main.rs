use intcode::Interpreter;
use itertools::Itertools;

fn main()
{
    let input = intcode::parse_file!("../input.txt");

    println!("{}", (0..50).cartesian_product(0..50).map(|c| test_coord(&input, c)).sum::<i64>());

    let mut y = 0;
    'outer: for x in 99 ..
    {
        loop
        {
            if      test_coord(&input, (x,    y   )) != 1 { y += 1 }
            else if test_coord(&input, (x-99, y+99)) != 1 { continue 'outer }
            else  { println!("{}", (x-99) * 10_000 + y);    break    'outer }
        }
    }
}

fn test_coord(input : &[i64], (x, y) : (i64, i64)) -> i64
{
    Interpreter::new(input.to_vec(), [x, y].iter().copied()).iter().next().unwrap()
}
