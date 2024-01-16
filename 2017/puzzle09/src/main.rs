fn main()
{
    let mut score   = 0;
    let mut count   = 0;
    let mut depth   = 0;
    let mut garbage = false;
    let mut negate  = false;

    for b in include_str!("../input.txt").trim_end().bytes()
    {
        if !garbage
        {
            match b
            {
                b'{' =>                   depth += 1,
                b'}' => { score += depth; depth -= 1 },
                b'<' => garbage = true,
                _    => ()
            }
        }
        else if !std::mem::replace(&mut negate, false)
        {
            match b
            {
                b'>' => garbage = false,
                b'!' => negate  = true,
                _    => count  += 1
            }
        }
    }

    println!("{score}");
    println!("{count}");
}
