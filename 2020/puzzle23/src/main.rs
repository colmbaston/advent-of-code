fn main()
{
    let input        = include_str!("../input.txt").trim_end();
    let mut cups_one = vec![0 ; input.len()+1];
    let mut prev     = 0;
    for i in input.bytes().map(|b| (b - b'0') as u32)
    {
        cups_one[prev] = i;
        prev           = i as usize;
    }
    let mut cups_two = cups_one.clone();

    cups_one[prev] = cups_one[0];
    crab_cups(&mut cups_one, 100);
    let mut current = cups_one[1];
    while current != 1
    {
        print!("{}", current);
        current = cups_one[current as usize];
    }
    println!();

    cups_two[prev] = input.len() as u32 + 1;
    cups_two.extend(input.len() as u32 + 2 ..= 1_000_000);
    cups_two.push(cups_two[0]);
    crab_cups(&mut cups_two, 10_000_000);
    println!("{}", { let x = cups_two[1]; x as u64 * cups_two[x as usize] as u64 });
}

fn crab_cups(cups : &mut [u32], iterations : u32)
{
    let mut current = cups[0] as usize;
    let mut claw    = Vec::with_capacity(3);

    for _ in 0 .. iterations
    {
        let mut next = current;
        for _ in 0 .. 3
        {
            next = cups[next] as usize;
            claw.push(next);
        }

        let mut destination = current;
        loop
        {
            destination = if destination == 1 { cups.len() - 1 } else { destination - 1 };
            if !claw.contains(&destination) { break }
        }

        cups[current]     = cups[claw[2]];
        cups[claw[2]]     = cups[destination];
        cups[destination] = claw[0] as u32;
        current           = cups[current] as usize;
        claw.clear();
    }

    cups[0] = current as u32;
}
