fn main()
{
    let reports = include_str!("../input.txt").lines()
                                              .map(|l| l.split_whitespace()
                                                        .map(|t| t.parse().unwrap())
                                                        .collect())
                                              .collect::<Vec<Vec<i32>>>();

    println!("{}", reports.iter().filter(|r| safe_one(r)).count());
    println!("{}", reports.iter().filter(|r| safe_two(r)).count());
}

fn safe_one(report : &[i32]) -> bool
{
    let (l, u) = if report[0] < report[1] { (1, 3) } else { (-3, -1) };

    report.windows(2)
          .map(|w| w[1] - w[0])
          .all(|d| l <= d && d <= u)
}

fn safe_two(report : &[i32]) -> bool
{
    (0 .. report.len()).any(|i|
    {
        let mut removed = report.to_vec();
        removed.remove(i);
        safe_one(&removed)
    })
}
