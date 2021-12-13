fn main()
{
    let layers : Vec<&[u8]> = include_str!("../input.txt").trim_end().as_bytes().chunks(25 * 6).collect();

    let count = |b : u8, bs : &[u8]| bs.iter().fold(0, |a, &x| a + (x == b) as u32);
    let layer = layers.iter().min_by_key(|bs| count(b'0', bs)).unwrap();

    println!("{}", count(b'1', layer) * count(b'2', layer));
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

    for a in image.iter()
    {
        for &b in a.iter()
        {
            print!("{}", if b == b'0' { ' ' } else { '#' });
        }
        println!();
    }
}
