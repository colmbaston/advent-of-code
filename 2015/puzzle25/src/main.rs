fn main()
{
    let (row, column) =
    {
        fn span(s : &str, p : impl Fn(char) -> bool) -> (&str, &str)
        {
            s.split_at(s.find(|c| !p(c)).unwrap_or(s.len()))
        }

        let          input  = include_str!("../input.txt");
        let (_,      input) = span(input, |c| !c.is_ascii_digit());
        let (row,    input) = span(input, |c|  c.is_ascii_digit());
        let (_,      input) = span(input, |c| !c.is_ascii_digit());
        let (column, _    ) = span(input, |c|  c.is_ascii_digit());

        (row.parse::<u32>().unwrap(), column.parse::<u32>().unwrap())
    };

    let mut code : u64 = 20151125;
    for _ in 0 .. (row * (row-1) + column * (column-1)) / 2 + row * (column-1)
    {
        code = (code * 252533) % 33554393;
    }
    println!("{}", code);
}
