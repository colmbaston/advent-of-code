use intcode;

fn main()
{
    let input = intcode::parse_file!("../input.txt");

    let mut memory = input.clone();
    memory[1] = 12;
    memory[2] = 2;
    intcode::interpret_noio(&mut memory);
    println!("{}", memory[0]);

    for i in 0..100
    {
        for j in 0..100
        {
            memory = input.clone();
            memory[1] = i;
            memory[2] = j;
            intcode::interpret_noio(&mut memory);

            if memory[0] == 19690720
            {
                println!("{}", 100 * i + j);
                return
            }
        }
    }
}
