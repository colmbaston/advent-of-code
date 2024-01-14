#![feature(array_windows)]

fn main()
{
    let mut phrases = include_str!("../input.txt").lines()
                                                  .map(|l| l.split_whitespace()
                                                            .map(|w| w.as_bytes().to_vec())
                                                            .collect::<Vec<Vec<u8>>>())
                                                  .collect::<Vec<Vec<Vec<u8>>>>();

    let mut count = 0;
    for phrase in phrases.iter_mut()
    {
        phrase.sort_unstable();
        count += phrase.array_windows().all(|[a, b]| a != b) as u32;
    }
    println!("{count}");

    count = 0;
    for phrase in phrases.iter_mut()
    {
        phrase.iter_mut().for_each(|word| word.sort_unstable());
        phrase.sort_unstable();
        count += phrase.array_windows().all(|[a, b]| a != b) as u32;
    }
    println!("{count}");
}
