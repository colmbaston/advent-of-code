fn main()
{
    let input = include_str!("../input.txt").lines()
                                            .map(|k| k.parse::<i32>().unwrap())
                                            .collect::<Vec<i32>>();

    let mut pc    = 0;
    let mut steps = 0;
    let mut jumps = input.clone();
    while let Some(j) = pc.try_into().ok().and_then(|i : usize| jumps.get_mut(i))
    {
        pc    += *j;
        *j    += 1;
        steps += 1
    }
    println!("{steps}");

    pc    = 0;
    steps = 0;
    jumps = input;
    while let Some(j) = pc.try_into().ok().and_then(|i : usize| jumps.get_mut(i))
    {
        pc    += *j;
        *j    += if *j >= 3 { -1 } else { 1 };
        steps += 1
    }
    println!("{steps}");
}
