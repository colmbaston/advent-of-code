fn main()
{
    let mut input = include_str!("../input.txt").lines().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    input.sort_unstable();

    for groups in 3 ..= 4
    {
        let target       = input.iter().sum::<u64>() / groups;
        let initial_size = input.iter().rev()
                                .try_fold((0, 0), |(l, s), x| if s < target { Ok((l+1, s+x)) } else { Err((l, s)) })
                                .unwrap_err().0;

        for size in initial_size ..
        {
            let mut cs = Vec::new();
            combinations(&input, size, target, 0, 1, &mut cs);

            if let Some(qe) = cs.iter().min()
            {
                println!("{}", qe);
                break;
            }
        }
    }
}

fn combinations(input : &[u64], size : usize, target : u64, sum : u64, product : u64, solutions : &mut Vec<u64>)
{
    if size == 0
    {
        if sum == target
        {
            solutions.push(product);
        }
    }
    else
    {
        for i in 0 ..= input.len() - size
        {
            if sum + input[i] > target { break }
            combinations(&input[i+1 ..], size - 1, target, sum + input[i], product * input[i], solutions);
        }
    }
}
