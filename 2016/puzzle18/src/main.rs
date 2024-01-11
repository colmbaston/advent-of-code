#![feature(array_windows)]

fn main()
{
    let mut row = vec![false];
    row.extend(include_str!("../input.txt").trim_end().bytes().map(|b| b == b'^'));
    row.push(false);

    let mut count  = 0;
    let mut buffer = Vec::new();
    for cycle in 0 ..
    {
        if      cycle ==      40 { println!("{count}")        }
        else if cycle == 400_000 { println!("{count}"); break }

        buffer.clear();
        buffer.push(false);
        buffer.extend(row.array_windows().map(|&[a, b, c]| { count += !b as u32; a ^ c }));
        buffer.push(false);

        std::mem::swap(&mut row, &mut buffer);
    }
}
