fn main()
{
    let input        = include_str!("../input.txt").trim_end();
    let mut cups_one = vec![0 ; input.len()+1];
    let mut prev     = 0;
    for i in input.bytes().map(|b| (b - b'0') as usize)
    {
        cups_one[prev] = i;
        prev           = i;
    }
    let mut cups_two = cups_one.clone();

    cups_one[prev] = cups_one[0];
    crab_cups(&mut cups_one, 100);
    let mut current = cups_one[1];
    while current != 1
    {
        print!("{}", current);
        current = cups_one[current];
    }
    println!();

    cups_two[prev] = input.len()+1;
    for i in input.len()+2 ..= 1_000_000
    {
        cups_two.push(i);
    }
    cups_two.push(cups_two[0]);
    crab_cups(&mut cups_two, 10_000_000);
    println!("{}", { let x = cups_two[1]; x * cups_two[x] });
}

fn crab_cups(cups : &mut [usize], iterations : usize)
{
    let mut current = cups[0];
    let mut claw    = Vec::with_capacity(3);

    for _ in 0 .. iterations
    {
        let mut next = current;
        for _ in 0 .. 3
        {
            next = cups[next];
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
        cups[destination] = claw[0];
        current           = cups[current];
        claw.clear();
    }

    cups[0] = current;
}
