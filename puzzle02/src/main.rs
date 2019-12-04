use intcode::interpret_intcode;

fn main()
{
    let input : Vec<i64> = std::fs::read_to_string("input.txt").unwrap().trim_end().split(',').map(|x| x.parse().unwrap()).collect();

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
