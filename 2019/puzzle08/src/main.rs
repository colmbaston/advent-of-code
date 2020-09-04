fn main()
{
    let input = include_str!("../input.txt");
    let layers : Vec<&[u8]> = input.trim_end().as_bytes().chunks(25 * 6).collect();

    let f = |x| bytecount::count(x, b'0');
    let m = layers.iter().min_by(|x, y| f(x).cmp(&f(y))).unwrap();

    println!("{}", bytecount::count(m, b'1') * bytecount::count(m, b'2'));
    decode_image(&layers);
}

fn decode_image(layers : &[&[u8]])
{
    let mut image = [[b'2' ; 25] ; 6];

    for l in layers.iter()
    {
        for (i, c) in l.iter().enumerate()
        {
            let pix = &mut image[i / 25][i % 25];
            if *pix == b'2' { *pix = *c }
        }
    }

    println!();
    image.iter().for_each(|a| { print!(" "); a.iter().for_each(|b| print!("{}", if *b == b'0' { ' ' } else { '#' })); println!() });
    println!();
}
