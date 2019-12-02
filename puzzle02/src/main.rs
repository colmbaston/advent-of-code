fn main()
{
    let input : Vec<i64> = std::fs::read_to_string("input.txt").unwrap().split(',').map(|x| x.trim_end().parse().unwrap()).collect();

    println!("{}", interpret_intcode(12, 2, &input));

    for i in 0..100
    {
        for j in 0..100
        {
            if interpret_intcode(i, j, &input) == 19690720
            {
                println!("{}", 100 * i + j);
                return
            }
        }
    }
}

fn interpret_intcode(noun : i64, verb : i64, code : &[i64]) -> i64
{
    let mut memory = code.to_vec();
    memory[1] = noun;
    memory[2] = verb;

    let mut ip = 0;
    loop
    {
        match memory.get(ip)
        {
            None     => break,
            Some(1)  => intcode_op(|x, y| x + y, &ip, &mut memory),
            Some(2)  => intcode_op(|x, y| x * y, &ip, &mut memory),
            Some(99) => break,
            Some(_)  => {}
        }
        ip += 4
    }

    memory[0]
}

fn intcode_op<F : Fn(i64, i64) -> i64>(f : F, ip : &usize, memory : &mut [i64])
{
    memory[memory[*ip+3] as usize] = f(memory[memory[*ip+1] as usize], memory[memory[*ip+2] as usize]);
}
