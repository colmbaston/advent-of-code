fn main()
{
    let (ops, rows) = parse_homework(include_str!("../input.txt"));

    let mut sum_one = 0;
    let mut sum_two = 0;
    for (i, op) in ops.into_iter().enumerate()
    {

        let row_wise = rows.iter().map(|row| trim_parse(row[i]));
        let col_wise = aoc::transpose::transpose(rows.iter().map(|row| row[i]))
                                  .map(|col| trim_parse(&col));

        if let b'+' = op
        {
            sum_one += row_wise.sum::<u64>();
            sum_two += col_wise.sum::<u64>();
        }
        else
        {
            sum_one += row_wise.product::<u64>();
            sum_two += col_wise.product::<u64>();
        }
    }
    println!("{sum_one}");
    println!("{sum_two}");
}

fn trim_parse(bytes : &[u8]) -> u64
{
    std::str::from_utf8(bytes).unwrap()
             .trim()
             .parse().unwrap()
}

fn parse_homework(s : &str) -> (Vec<u8>, Vec<Vec<&[u8]>>)
{
    let mut ls      = s.lines();
    let mut ops     = Vec::new();
    let mut widths  = Vec::new();
    let mut op_line = ls.next_back().unwrap().as_bytes();
    loop
    {
        ops.push(op_line[0]);
        match op_line[1 ..].iter().position(|&b| b != b' ')
        {
            None    => { widths.push(op_line.len()); break },
            Some(i) => { widths.push(i); op_line = &op_line[i+1 ..] }
        }
    }

    let mut rows = Vec::new();
    for l in ls
    {
        let mut l    = l.as_bytes();
        let mut cols = Vec::new();
        for &w in widths.iter()
        {
            cols.push(&l[.. w]);
            if l.len() > w { l = &l[w+1 ..] }
        }
        rows.push(cols);
    }

    (ops, rows)
}
