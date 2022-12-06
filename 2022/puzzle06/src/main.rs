use std::collections::HashSet;

fn main()
{
    let mut set = HashSet::new();
    for size in [4, 14]
    {
        for (window, ix) in include_str!("../input.txt").trim_end().as_bytes().windows(size).zip(size ..)
        {
            set.clear();
            set.extend(window.iter().copied());
            if set.len() == window.len() { println!("{ix}"); break }
        }
    }
}
