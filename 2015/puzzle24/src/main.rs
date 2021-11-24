fn main()
{
    let input = include_str!("../input.txt").lines().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    for groups in 3 ..= 4
    {
        let target     = input.iter().sum::<u64>() / groups;
        let target_len = input.iter()
                              .rev()
                              .try_fold((0, 0), |(l, s), x| if s < target { Ok((l+1, s+x)) } else { Err((l, s)) })
                              .unwrap_err().0;

        for len in target_len ..
        {
            let mut cs = Vec::new();
            combinations(&input, len, Vec::new(), &mut cs);

            if let Some(qe) = cs.into_iter()
                                .filter_map(|v| if v.iter().sum::<u64>() == target { Some(v.iter().product::<u64>()) } else { None })
                                .min()
            {
                println!("{}", qe);
                break;
            }
        }
    }
}

fn combinations(input : &[u64], target_len : usize, partial : Vec<u64>, output : &mut Vec<Vec<u64>>)
{
    if let 0 = target_len
    {
        output.push(partial);
    }
    else
    {
        for i in 0 ..= input.len() - target_len
        {
            let mut current = partial.clone();
            current.push(input[i]);
            combinations(&input[i+1 ..], target_len-1, current, output);
        }
    }
}
