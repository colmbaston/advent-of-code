fn main()
{
    let equations = include_str!("../input.txt").lines().map(parse_equation).collect::<Vec<(u64, Vec<u64>)>>();
    let mut ops   = vec![|x, y| x + y, |x, y| x * y];
    println!("{}", equations.iter().filter_map(|(res, inputs)| eval(*res, inputs[0], &inputs[1 ..], &ops).then_some(*res)).sum::<u64>());
    ops.push(|x, y| x * 10_u64.pow(y.ilog10()+1) + y);
    println!("{}", equations.iter().filter_map(|(res, inputs)| eval(*res, inputs[0], &inputs[1 ..], &ops).then_some(*res)).sum::<u64>());
}

fn parse_equation(s : &str) -> (u64, Vec<u64>)
{
    let (a, b) = s.split_once(": ").unwrap();
    (a.parse().unwrap(), b.split_whitespace().map(|t| t.parse().unwrap()).collect())
}

fn eval(target : u64, acc : u64, inputs : &[u64], ops : &[impl Fn(u64, u64) -> u64]) -> bool
{
    if acc > target { return false }

    match inputs.split_first()
    {
        None             => acc == target,
        Some((&k, rest)) => ops.iter().any(|op| eval(target, op(acc, k), rest, ops))
    }
}
