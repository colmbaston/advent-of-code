fn main()
{
    let input = include_str!("../input.txt").lines().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let mut one = None;
    let mut two = None;

    'outer: for (i, x) in input.iter().enumerate()
    {
        for (j, y) in input.iter().enumerate().skip(i)
        {
            if x + y == 2020 { one = Some(x * y) }

            for z in input.iter().skip(j)
            {
                if x + y + z == 2020 { two = Some(x * y * z) }
            }

            if one.and(two).is_some() { break 'outer }
        }
    }

    println!("{}", one.unwrap());
    println!("{}", two.unwrap());
}
