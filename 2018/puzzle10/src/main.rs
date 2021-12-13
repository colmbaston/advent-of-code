use std::collections::HashSet;

fn main()
{
    let mut points = include_str!("../input.txt").lines().map(Point::parse).collect::<Vec<_>>();

    let mut last_rect = aoc::bounds::bounds_2d(points.iter().map(|p| &p.position)).unwrap();
    let mut last_area = rect_area(&last_rect);

    // simulate until the area of the bounding rectangle stops contracting
    for step in 0 ..
    {
        points.iter_mut().for_each(Point::step_forwards);
        let rect = aoc::bounds::bounds_2d(points.iter().map(|p| &p.position)).unwrap();
        let area = rect_area(&rect);

        if area > last_area
        {
            // the previous iteration was the (local) minimum, so step_backwards
            points.iter_mut().for_each(Point::step_backwards);

            // part 1: display the state of area inside the bounding rectangle
            let (min_x, min_y, max_x, max_y) = last_rect;
            let canvas = points.into_iter().map(|p| p.position).collect::<HashSet<(i64, i64)>>();
            println!();
            for y in min_y ..= max_y
            {
                print!(" ");
                for x in min_x ..= max_x
                {
                    print!("{}", if canvas.contains(&(x, y)) { '#' } else { ' ' });
                }
                println!();
            }
            println!();

            // part 2: print which step it was
            println!("{}", step);
            break
        }

        last_rect = rect;
        last_area = area;
    }
}

struct Point
{
    position: (i64, i64),
    velocity: (i64, i64)
}

impl Point
{
    fn parse(s : &str) -> Point
    {
        fn span_integer(s : &str) -> (&str, &str)
        {
            s.split_at(s.find(|c : char| !(c.is_ascii_digit() || c == '-')).unwrap_or_else(|| s.len()))
        }

        let (px, s) = span_integer(s[10..].trim_start());
        let (py, s) = span_integer(s[ 2..].trim_start());
        let (vx, s) = span_integer(s[12..].trim_start());
        let (vy, _) = span_integer(s[ 2..].trim_start());

        Point
        {
            position: (px.parse().unwrap(), py.parse().unwrap()),
            velocity: (vx.parse().unwrap(), vy.parse().unwrap())
        }
    }

    fn step_forwards(&mut self)
    {
        let Point { position: (px, py), velocity: (vx, vy) } = self;

        *px += *vx;
        *py += *vy;
    }

    fn step_backwards(&mut self)
    {
        let Point { position: (px, py), velocity: (vx, vy) } = self;

        *px -= *vx;
        *py -= *vy;
    }
}

fn rect_area((min_x, min_y, max_x, max_y) : &(i64, i64, i64, i64)) -> u64
{
    ((max_x - min_x) * (max_y - min_y)) as u64
}
