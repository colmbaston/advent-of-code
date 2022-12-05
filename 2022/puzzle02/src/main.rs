fn main()
{
    let (one, two) = include_bytes!("../input.txt").chunks(4)
                                                   .fold((0, 0), |(one, two), arr|
                                                   {
                                                       let a = (arr[0] - b'A') as i8;
                                                       let b = (arr[2] - b'X') as i8;

                                                       (one + score(a, b)               as u32,
                                                        two + score(a, (2 + a + b) % 3) as u32)
                                                   });

    println!("{one}");
    println!("{two}");
}

fn score(a : i8, b : i8) -> i8
{
    1 + b + ((a - b).rem_euclid(3) * 2 + 1) % 3 * 3
}
