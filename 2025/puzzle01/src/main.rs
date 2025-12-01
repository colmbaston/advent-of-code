fn main()
{
    let mut dial      = 50;
    let mut count_one = 0;
    let mut count_two = 0;
    for line in include_str!("../input.txt").lines()
    {
        let right  = line.as_bytes()[0] == b'R';
        let amount = line[1..].parse::<i32>().unwrap();

        count_two += amount / 100;
        let rem    = amount % 100;
        if right && dial + rem >= 100 || !right && dial != 0 && dial - rem <= 0 { count_two += 1 }

        dial += if right { rem } else { -rem };
        dial  = dial.rem_euclid(100);
        if dial == 0 { count_one += 1 }
    }
    println!("{count_one}");
    println!("{count_two}")
}
