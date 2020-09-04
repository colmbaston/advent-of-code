use std::iter::once;
use intcode::Interpreter;

fn main()
{
    let input = intcode::parse_file!("../input.txt");
    println!("{}", Interpreter::new(input.clone(), once(1)).iter().next().unwrap());
    println!("{}", Interpreter::new(input,         once(2)).iter().next().unwrap());
}
