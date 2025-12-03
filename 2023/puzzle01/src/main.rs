fn main()
{
    let input = include_str!("../input.txt");
    println!("{}", input.lines().map(calibration_one).sum::<u32>());
    println!("{}", input.lines().map(calibration_two).sum::<u32>());
}

fn calibration_one(line : &str) -> u32
{
    let mut digits = line.bytes().filter(|b| b.is_ascii_digit()).map(|b| b - b'0');

    let first = digits.next().unwrap_or(0);
    let last  = digits.next_back().unwrap_or(first);

    10 * first as u32 + last as u32
}

const SPELLINGS : [&str ; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn calibration_two(line : &str) -> u32
{
    let mut substring = line.as_bytes();
    let first = 'outer: loop
    {
        let (first, rest) = substring.split_first().unwrap();
        if first.is_ascii_digit() { break first - b'0' }

        for (s, i) in SPELLINGS.iter().zip(1..)
        {
            if substring.strip_prefix(s.as_bytes()).is_some() { break 'outer i }
        }

        substring = rest;
    };

    substring = line.as_bytes();
    let last = 'outer: loop
    {
        let (last, rest) = substring.split_last().unwrap();
        if last.is_ascii_digit() { break last - b'0' }

        for (s, i) in SPELLINGS.iter().zip(1..)
        {
            if substring.strip_suffix(s.as_bytes()).is_some() { break 'outer i }
        }

        substring = rest;
    };

    10 * first as u32 + last as u32
}
