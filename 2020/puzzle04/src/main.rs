fn main()
{
    let input = parse_passports(include_str!("../input.txt"));

    let all_present = input.iter().filter(|p| p.iter().filter(|(k, _)| k != "cid").count() == 7);

    println!("{}", all_present.clone().count());
    println!("{}", all_present.filter(|p| p.iter().all(valid_field)).count());
}

type Passport = Vec<(String, String)>;

fn parse_passports(s : &str) -> Vec<Passport>
{
    let mut ps = Vec::new();

    for p in s.split("\n\n")
    {
        let mut fs = Vec::new();
        for f in p.split_whitespace()
        {
            let mut x = f.split(':');
            fs.push((x.next().unwrap().to_string(),
                     x.next().unwrap().to_string()));
        }
        ps.push(fs);
    }

    ps
}

fn valid_field((k, v) : &(String, String)) -> bool
{
    let n_digits = |n| if v.len() == n { v.parse::<u32>().ok() } else { None };

    match k.as_str()
    {
        "byr" => n_digits(4).map(|k| 1920 <= k && k <= 2002),
        "iyr" => n_digits(4).map(|k| 2010 <= k && k <= 2020),
        "eyr" => n_digits(4).map(|k| 2020 <= k && k <= 2030),
        "hgt" => match v.strip_suffix("cm")
        {
            Some(cm) =>                                  cm.parse::<u32>().ok().map(|k| 150 <= k && k <= 193),
            None     => v.strip_suffix("in").and_then(|i| i.parse::<u32>().ok().map(|k|  59 <= k && k <=  76))
        },
        "hcl" => v.strip_prefix('#').map(|hcl| hcl.bytes().all(|c| c.is_ascii_digit() || b'a' <= c && c <= b'f')),
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().find(|s| **s == v.as_str()).map(|_| true),
        "pid" => n_digits(9).map(|_| true),
        _     => Some(true),
    }
    .unwrap_or(false)
}
