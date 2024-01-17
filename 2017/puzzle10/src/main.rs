fn main()
{
    let input = include_str!("../input.txt").trim_end();

    println!("{}", knot_hash::rounds(1, &input.split(',')
                                   .map(|k| k.parse().unwrap())
                                   .collect::<Vec<u8>>()).into_iter().map(|k| k as u32)
                                                         .take(2).product::<u32>());

    knot_hash::hash(input.as_bytes()).iter().for_each(|b| print!("{b:x}"));
    println!();
}
