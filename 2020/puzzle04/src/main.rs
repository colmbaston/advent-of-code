fn main()
{
    let input = parse_passports(include_str!("../input.txt"));

    let all_present = input.iter().filter(|p| p.iter().filter(|(k, _)| *k != "cid").count() == 7);

    println!("{}", all_present.clone().count());
    println!("{}", all_present.filter(|p| p.iter().all(valid_field)).count());
}

type Passport<'a> = Vec<(&'a str, &'a str)>;

fn parse_passports(s : &str) -> Vec<Passport>
{
    s.split("\n\n").map(|p|
    {
        p.split_whitespace().map(|f|
        {
            let mut it = f.split(':');

            (it.next().unwrap(), it.next().unwrap())
        })
        .collect()
    })
    .collect()
}

fn valid_field((k, v) : &(&str, &str)) -> bool
{
    let n_digits = |n| if v.len() == n { v.parse::<u32>().ok() } else { None };

    match *k
    {
        "byr" => n_digits(4).map(|n| (1920 ..= 2002).contains(&n)),
        "iyr" => n_digits(4).map(|n| (2010 ..= 2020).contains(&n)),
        "eyr" => n_digits(4).map(|n| (2020 ..= 2030).contains(&n)),
        "hgt" => match v.strip_suffix("cm")
        {
            Some(cm) =>                                  cm.parse::<u8>().ok().map(|n| (150 ..= 193).contains(&n)),
            None     => v.strip_suffix("in").and_then(|i| i.parse::<u8>().ok().map(|n| (59  ..=  76).contains(&n)))
        },
        "hcl" => v.strip_prefix('#').map(|hcl| hcl.bytes().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())),
        "ecl" => Some(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(v)),
        "pid" => n_digits(9).map(|_| true),
        _     => Some(true),
    }
    .unwrap_or(false)
}
