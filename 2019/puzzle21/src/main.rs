use intcode::Interpreter;

fn main()
{
    let input = intcode::parse_file!("../input.txt");

    let walk_script = "NOT C J\n\
                       AND D J\n\
                       NOT A T\n\
                       OR  T J\n\
                       WALK\n";

    let run_script  = "OR  E J\n\
                       OR  H J\n\
                       AND D J\n\
                       OR  A T\n\
                       AND B T\n\
                       AND C T\n\
                       NOT T T\n\
                       AND T J\n\
                       RUN\n";

    println!("{}", Interpreter::new(input.clone(), walk_script.bytes().map(|b| b as i64)).iter().last().unwrap());
    println!("{}", Interpreter::new(input,          run_script.bytes().map(|b| b as i64)).iter().last().unwrap());
}
