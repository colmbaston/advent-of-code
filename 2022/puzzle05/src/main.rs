fn main()
{
    let mut input      = include_str!("../input.txt").split("\n\n");
    let mut stacks_one = parse_stacks(input.next().unwrap_or(""));
    let mut stacks_two = stacks_one.clone();

    let mut buffer = Vec::new();
    for (count, from, to) in input.next().unwrap_or("").lines().map(parse_instruction)
    {
        for (stacks, one) in [&mut stacks_one, &mut stacks_two].iter_mut().zip([true, false])
        {
            stacks.get_mut(from)
                  .map(|s| { let d = s.drain(s.len() - count ..);
                             if one { buffer.extend(d.rev()) }
                             else   { buffer.extend(d)       }})
                  .and_then(|_| stacks.get_mut(to)
                                      .map(|s| s.append(&mut buffer)));
        }
    }

    for stacks in &[stacks_one, stacks_two]
    {
        for s in stacks.iter()
        {
            if let Some(&b) = s.last() { print!("{}", b as char) }
        }
        println!();
    }
}

fn parse_stacks(s : &str) -> Vec<Vec<u8>>
{
    let mut layers = s.lines().rev();
    let mut stacks = Vec::new();
    stacks.extend(layers.next().unwrap_or("")
                        .split_whitespace()
                        .map(|_| Vec::new()));

    for l in layers
    {
        for (chunk, stack) in l.as_bytes().chunks(4).zip(stacks.iter_mut())
        {
            if let Some(&label) = chunk.get(1).filter(|label| !label.is_ascii_whitespace())
            {
                stack.push(label);
            }
        }
    }

    stacks
}

fn parse_instruction(s : &str) -> (usize, usize, usize)
{
    let mut nums = s.split_whitespace()
                    .skip(1).step_by(2)
                    .filter_map(|word| word.parse::<usize>().ok());

    let count = nums.next().unwrap_or(0);
    let from  = nums.next().unwrap_or(1) - 1;
    let to    = nums.next().unwrap_or(1) - 1;

    (count, from, to)
}
