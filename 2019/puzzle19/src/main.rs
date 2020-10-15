use intcode::Interpreter;

fn main()
{
    let input = intcode::parse_file!("../input.txt");

    println!("{}", (0..50).fold(0, |sum, x| (0..50).fold(sum, |a, y| a + test_coord(&input, (x, y)))));

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
