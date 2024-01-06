use md5::{ Md5, Digest};

fn main()
{
    let input = include_str!("../input.txt").trim_end();

    let mut toggle = false;
    for i in 1 ..
    {
        let mut hasher = Md5::new();
        hasher.update(input);
        hasher.update(format!("{i}"));
        let hash = hasher.finalize();

        if hash[0] == 0x00
        && hash[1] == 0x00
        && hash[2] <= if toggle { 0x00 } else { 0x0f }
        {
            println!("{}", i);

            if toggle { break }
            toggle = true;
        }
    }
}
