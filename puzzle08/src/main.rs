fn main()
{
    let input = include_str!("../input.txt");
    let layers : Vec<&[u8]> = input.trim_end().as_bytes().chunks(25 * 6).collect();

    let f = |x : &[u8]| x.iter().filter(|y| **y == b'0').count();
    let m = layers.iter().min_by(|x, y| f(x).cmp(&f(y))).unwrap();

    println!("{}", m.iter().filter(|x| **x == b'1').count() * m.iter().filter(|x| **x == b'2').count());
    decode_image(&layers);
}

fn decode_image(layers : &Vec<&[u8]>)
{
    let mut image = [[b'2' ; 6] ; 25];

    for l in layers.iter()
    {
        for (i, c) in l.iter().enumerate()
        {
            let pix = &mut image[i % 25][i / 25];
            if *pix == b'2' { *pix = *c }
        }
    }

    println!();
    for y in 0 .. 6
    {
        print!(" ");
        for x in 0 .. 25
        {
            print!("{}", if image[x][y] == b'0' { ' ' } else { '#' });
        }
        println!()
    }
    println!();
}
