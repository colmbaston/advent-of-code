use ::intcode::intcode;

fn main()
{
    let mut code = intcode::parse_file("input.txt");
    intcode::interpret(&mut code);
}
