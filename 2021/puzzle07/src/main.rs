fn main()
{
    let input = include_str!("../input.txt").trim_end().split(',').map(|s| s.parse().unwrap()).collect::<Vec<i32>>();

    let fuel_one   = |a : i32, b : i32| {         (a - b).abs()                  };
    let fuel_two   = |a : i32, b : i32| { let c = (a - b).abs(); (c * (c+1)) / 2 };
    let (min, max) = input.iter().fold((i32::MAX, i32::MIN), |(min, max), &k| (min.min(k), max.max(k)));

    for fuel in [fuel_one, fuel_two]
    {
        println!("{}", (min ..= max).map(|a| input.iter().map(|&b| fuel(a, b)).sum::<i32>()).min().unwrap());
    }
}
