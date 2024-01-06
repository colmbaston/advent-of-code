use md5::{ Md5, Digest };

fn main()
{
    let input = include_str!("../input.txt").trim_end();

    let mut password_one = Vec::new();
    let mut password_two = [None ; 8];
    for i in 0 ..
    {
        let mut hasher = Md5::new();
        hasher.update(input);
        hasher.update(format!("{i}"));
        let hash = hasher.finalize();

        if hash[0] == 0x00
        && hash[1] == 0x00
        && hash[2] <= 0x0f
        {
            if password_one.len() < 8
            {
                password_one.push(hash[2])
            }

            let pos = hash[2] as usize;
            if pos < 8
            {
                password_two[pos].get_or_insert(hash[3] >> 4);
            }

            if password_one.len() == 8 && password_two.iter().all(|o| o.is_some())
            {
                break
            }
        }
    }

    for digit in password_one { print!("{digit:x}") }
    println!();
    for digit in password_two { print!("{:x}", digit.unwrap_or(0)) }
    println!();
}
