use intcode::Interpreter;

fn main()
{
    let input  = intcode::parse_file!("../input.txt");
    let script = include_str!("../script.txt").bytes().map(|b| b as i64);
    let output = Interpreter::new(input.clone(), script).iter().map(|x| x as u8 as char).collect::<String>();
    println!("{}", output.lines().last().unwrap());
}
