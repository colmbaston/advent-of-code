use std::iter::empty;
use intcode::Interpreter;

fn main()
{
    let input = intcode::parse_file!("../input.txt");

    let mut interpreter = Interpreter::new(input.clone(), empty());
    interpreter.memory[1] = 12;
    interpreter.memory[2] = 2;
    interpreter.iter().next();
    println!("{}", interpreter.memory[0]);

    for i in 0 .. 100
    {
        for j in 0 .. 100
        {
            interpreter = Interpreter::new(input.clone(), empty());
            interpreter.memory[1] = i;
            interpreter.memory[2] = j;
            interpreter.iter().next();

            if interpreter.memory[0] == 19_690_720
            {
                println!("{}", 100 * i + j);
                return
            }
        }
    }
}
