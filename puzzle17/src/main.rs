use intcode::Interpreter;
use std::collections::HashSet;

fn main()
{
    let mut input = intcode::parse_file!("../input.txt");

    let scaffold = Interpreter::new(input.clone(), std::iter::empty()).iter().fold(((0, 0), HashSet::new()), |((x, y), mut s), p|
    {
        if p == 10
        {
            return ((0, y+1), s)
        }
        else if p != 46
        {
            s.insert((x, y));
        }

        ((x+1, y), s)
    })
    .1;

    println!("{}", scaffold.iter().fold(0, |a, &(x, y)|
    {
        for c in [(x+1, y), (x-1, y), (x, y+1), (x, y-1)].iter()
        {
            if !scaffold.contains(c) { return a }
        }

        a + x * y
    }));

    // handcrafted for my input, for now
    let vacuum_program = "A,B,A,C,B,A,C,B,A,C\n\
                          L,6,L,4,R,12\n\
                          L,6,R,12,R,12,L,8\n\
                          L,6,L,10,L,10,L,6\n\
                          n\n"
                         .bytes().map(|b| b as i64);

    input[0] = 2;
    println!("{}", Interpreter::new(input, vacuum_program).last().unwrap());
}
