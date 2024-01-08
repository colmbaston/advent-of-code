fn main()
{
    let mut screen = [[false ; 50] ; 6];

    for inst in include_str!("../input.txt").lines()
    {
        if let Some(rest) = inst.strip_prefix("rect ")
        {
            let (x, y) = rest.split_once('x').unwrap();
            let (x, y) = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());

            for row in screen.iter_mut().take(y)
            {
                row[.. x].fill(true)
            }
        }
        else if let Some(rest) = inst.strip_prefix("rotate row y=")
        {
            let (y, x) = rest.split_once(" by ").unwrap();
            let (x, y) = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());

            screen[y].rotate_right(x);
        }
        else if let Some(rest) = inst.strip_prefix("rotate column x=")
        {
            let (x, y) = rest.split_once(" by ").unwrap();
            let (x, y) = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());

            let mut column = screen.iter().map(|row| row[x]).collect::<Vec<bool>>();
            column.rotate_right(y);
            for (row, pixel) in screen.iter_mut().zip(column.into_iter())
            {
                row[x] = pixel
            }
        }
        else
        {
            unreachable!()
        }
    }

    println!("{}", screen.into_iter()
                         .map(|row| row.into_iter()
                                       .filter(|&pixel| pixel)
                                       .count())
                         .sum::<usize>());

    for row in screen.iter()
    {
        for &pixel in row.iter()
        {
            print!("{}", if pixel { '#' } else { ' ' });
        }
        println!()
    }
}
