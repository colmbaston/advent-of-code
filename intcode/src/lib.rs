pub mod intcode
{
    use std::io::{ stdin, stdout, Write };

    pub fn parse_file(fp : &str) -> Vec<i64>
    {
        std::fs::read_to_string(fp).unwrap().trim_end().split(',').map(|x| x.parse().unwrap()).collect()
    }

    pub fn interpret(memory : &mut [i64])
    {
        let mut ip = 0;
        loop
        {
            let (opcode, modes) = match memory.get(ip)
            {
                None    => { println!("overran the code buffer, terminating!"); break }
                Some(k) => decode(k)
            };

            match opcode
            {
                01 => binop(|x, y| x + y,                      &modes, &mut ip, memory),
                02 => binop(|x, y| x * y,                      &modes, &mut ip, memory),
                03 => input(                                   &modes, &mut ip, memory),
                04 => output(                                  &modes, &mut ip, memory),
                05 => jump(|x| x != 0,                         &modes, &mut ip, memory),
                06 => jump(|x| x == 0,                         &modes, &mut ip, memory),
                07 => binop(|x, y| if x <  y { 1 } else { 0 }, &modes, &mut ip, memory),
                08 => binop(|x, y| if x == y { 1 } else { 0 }, &modes, &mut ip, memory),
                99 => break,
                _  => { println!("invalid opcode: {}, terminating!", opcode); break }
            };
        }
    }

    type Modes = Vec<bool>;

    fn decode(i : &i64) -> (i64, Modes)
    {
        let opcode = i % 100;
        let mut k  = i / 100;

        let args = match opcode
        {
            01 => 3,
            02 => 3,
            03 => 1,
            04 => 1,
            05 => 2,
            06 => 2,
            07 => 3,
            08 => 3,
            _  => 0
        };
        let modes  = (0 .. args).map(|_| if *(&k) == 0 { false } else { let m = *(&k) % 10 == 1; *(&mut k) /= 10; m }).collect();

        (opcode, modes)
    }

    fn binop<F : Fn(i64, i64) -> i64>(op : F, modes : &Modes, ip : &mut usize, memory : &mut [i64])
    {
        *index_modal(modes[2], *ip+3, memory) = op(*index_modal(modes[0], *ip+1, memory), *index_modal(modes[1], *ip+2, memory));
        *ip += 4
    }

    fn input(modes : &Modes, ip : &mut usize, memory : &mut [i64])
    {
        loop
        {

            print!("input: ");
            stdout().flush();

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            match input.trim_end().parse()
            {
                Ok(x)  => { *index_modal(modes[0], *ip+1, memory) = x; break }
                Err(_) => println!("parse error!")
            }
        }
        *ip += 2
    }

    fn output(modes : &Modes, ip : &mut usize, memory : &mut [i64])
    {
        println!("{}", *index_modal(modes[0], *ip+1, memory));
        *ip += 2
    }

    fn jump<F : Fn(i64) -> bool>(f : F, modes : &Modes, ip : &mut usize, memory : &mut [i64])
    {
        if f(*index_modal(modes[0], *ip+1, memory))
        {
            *ip = *index_modal(modes[1], *ip+2, memory) as usize
        }
        else
        {
            *ip += 3
        }
    }

    fn index_modal(immediate : bool, ix : usize, memory : &mut [i64]) -> &mut i64
    {
        &mut memory[if immediate { ix } else { memory[ix] as usize }]
    }
}
