fn main()
{
    let mut one = 0;
    let mut two = None;

    for (b, i) in include_str!("../input.txt").bytes().zip(1..)
    {
        match b
        {
            b'(' => one += 1,
            b')' => one -= 1,
            _    => break
        }

        if let (-1, None) = (one, two)
        {
            two = Some(i)
        }
    }

    println!("{}", one);
    println!("{}", two.unwrap());
}
