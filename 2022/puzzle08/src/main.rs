use itertools::Itertools;
use std::collections::HashSet;

fn main()
{
    let trees_by_row = include_str!("../input.txt").lines()
                                                   .map(|l| l.as_bytes())
                                                   .collect::<Vec<&[u8]>>();
    let trees_by_col = aoc::transpose::transpose(&trees_by_row);

    let mut visible = HashSet::new();
    for (y, row) in trees_by_row.iter().enumerate()
    {
        let row_iter = row.iter().copied().enumerate();
        visible.extend(       visible_indices(row_iter.clone())
                       .chain(visible_indices(row_iter.rev()))
                       .map(|x| (x, y)));
    }
    for (x, col) in trees_by_col.iter().enumerate()
    {
        let col_iter = col.iter().copied().enumerate();
        visible.extend(       visible_indices(col_iter.clone())
                       .chain(visible_indices(col_iter.rev()))
                       .map(|y| (x, y)));
    }
    println!("{}", visible.len());

    let mut score = 0;
    for (y, row) in trees_by_row.iter().enumerate().skip(1).take(trees_by_row.len()-2)
    {
        for ((x, col), &height) in trees_by_col.iter().enumerate().skip(1).take(trees_by_col.len()-2).zip(row.iter().skip(1))
        {
            score = score.max(viewing_distance(height, col[..   y].iter().copied().rev())
                            * viewing_distance(height, col[y+1 ..].iter().copied())
                            * viewing_distance(height, row[..   x].iter().copied().rev())
                            * viewing_distance(height, row[x+1 ..].iter().copied()));
        }
    }
    println!("{score}");
}

fn visible_indices(iter : impl Iterator<Item = (usize, u8)>) -> impl Iterator<Item = usize>
{
    let mut max = 0;
    iter.filter_map(move |(i, b)|
    {
        if b > max { max = b; Some(i) } else { None }
    })
}

fn viewing_distance(height : u8, iter : impl Iterator<Item = u8>) -> usize
{
    let mut iter = iter.peekable();
    iter.peeking_take_while(|&h| h < height).count() + iter.next().is_some() as usize
}
