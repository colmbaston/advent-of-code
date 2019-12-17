use intcode;
use std::collections::HashMap;

fn main()
{
    let mut code1 = intcode::parse_file!("../input.txt");
    let mut code2 = code1.clone();
    code2[0] = 2;

    let camera = intcode::interpret_vecio(&mut code1, Vec::new()).iter().fold(((0, 0), HashMap::new()), |((x, y), mut m), p|
    {
        match *p as u8
        {
            b'\n' => ((0, y+1), m),
            b'.'  => ((x+1, y), m),
            b     => { m.insert((x, y), b); ((x+1, y), m) }
        }
    })
    .1;

    println!("{}", camera.keys().fold(0, |a, (x, y)|
    {
        for c in ortho((*x, *y)).iter()
        {
            if camera.get(&c).is_none() { return a }
        }

        a + x * y
    }));

    // handcrafted for my input, for now
    let vacuum_program = "A,B,A,C,B,A,C,B,A,C\n\
                          L,6,L,4,R,12\n\
                          L,6,R,12,R,12,L,8\n\
                          L,6,L,10,L,10,L,6\n\
                          n\n"
                          .bytes().map(|b| b as i64).collect();

    println!("{}", intcode::interpret_vecio(&mut code2, vacuum_program).last().unwrap());
}

fn ortho((x, y) : (i64, i64)) -> [(i64, i64) ; 4]
{
    [(x+1, y), (x-1, y), (x, y+1), (x, y-1)]
}
