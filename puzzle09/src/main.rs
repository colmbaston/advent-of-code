use intcode;

fn main()
{
    let mut code1 = intcode::parse_file!("../input.txt");
    let mut code2 = code1.clone();

    println!("{}", intcode::interpret_vecio(&mut code1, vec![1])[0]);
    println!("{}", intcode::interpret_vecio(&mut code2, vec![2])[0]);
}
