fn main()
{
    let input = include_str!("../input.txt").lines().map(parse_inst).collect::<Vec<Inst>>();

    let mut lights;
    for part in [one, two].iter()
    {
        lights = [[0 ; 1000] ; 1000];
        input.iter().for_each(|i| run(i, part, &mut lights));
        println!("{}", lights.iter().map(|r| r.iter().map(|&x| x as u32).sum::<u32>()).sum::<u32>());
    }
}

fn run(i : &Inst, f : impl Fn(&Mode) -> Box<dyn Fn(&mut u8)>, lights : &mut [[u8 ; 1000] ; 1000])
{
    let g = f(&i.mode);

    for r in lights.iter_mut().skip(i.lx).take(1 + i.ux - i.lx)
    {
        for x in r.iter_mut().skip(i.ly).take(1 + i.uy - i.ly)
        {
            g(x)
        }
    }
}

fn one(m : &Mode) -> Box<dyn Fn(&mut u8)>
{
    Box::new(match m
    {
        Mode::On     => |x : &mut u8| *x = 1,
        Mode::Off    => |x : &mut u8| *x = 0,
        Mode::Toggle => |x : &mut u8| *x = (*x+1) % 2
    })
}

fn two(m : &Mode) -> Box<dyn Fn(&mut u8)>
{
    Box::new(match m
    {
        Mode::On     => |x : &mut u8| *x += 1,
        Mode::Off    => |x : &mut u8| *x = x.saturating_sub(1),
        Mode::Toggle => |x : &mut u8| *x += 2
    })
}

struct Inst
{
    mode : Mode,
    lx   : usize,
    ly   : usize,
    ux   : usize,
    uy   : usize
}

enum Mode
{
    On,
    Off,
    Toggle
}

fn parse_inst(s : &str) -> Inst
{
    let mut i = s.split([' ', ',']);

    let mode = match i.next()
    {
        Some("turn")   => if let Some("on") = i.next() { Mode::On } else { Mode::Off },
        Some("toggle") => Mode::Toggle,
        _              => unreachable!()
    };

    Inst
    {
        mode,
        lx:             i.next().unwrap().parse().unwrap(),
        ly:             i.next().unwrap().parse().unwrap(),
        ux: { i.next(); i.next().unwrap().parse().unwrap() },
        uy:             i.next().unwrap().parse().unwrap()
    }
}
