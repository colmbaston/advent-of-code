use std::collections::HashMap;

fn main()
{
    let input = include_str!("../input.txt").lines().map(Inst::parse).collect::<Vec<Inst>>();

    let mut cache = HashMap::new();
    println!("{}", reverse_digits(search(&input, 0, [0 ; 4], (1 ..= 9).rev(), &mut cache).unwrap()));
    cache.clear();
    println!("{}", reverse_digits(search(&input, 0, [0 ; 4],  1 ..= 9,        &mut cache).unwrap()));
}

enum Inst
{
    Inp(Var),
    Add(Var, Arg),
    Mul(Var, Arg),
    Div(Var, Arg),
    Mod(Var, Arg),
    Eql(Var, Arg)
}

enum Var { W, X, Y, Z }

enum Arg
{
    Var(Var),
    Int(i64)
}

impl Inst
{
    fn parse(s : &str) -> Inst
    {
        match s.split_whitespace().collect::<Vec<&str>>()[..]
        {
            ["inp", a]    => Inst::Inp(Var::parse(a)),
            ["add", a, b] => Inst::Add(Var::parse(a), Arg::parse(b)),
            ["mul", a, b] => Inst::Mul(Var::parse(a), Arg::parse(b)),
            ["div", a, b] => Inst::Div(Var::parse(a), Arg::parse(b)),
            ["mod", a, b] => Inst::Mod(Var::parse(a), Arg::parse(b)),
            ["eql", a, b] => Inst::Eql(Var::parse(a), Arg::parse(b)),
            _             => unreachable!()
        }
    }
}

impl Var
{
    fn parse(s : &str) -> Var
    {
        match s
        {
            "w" => Var::W,
            "x" => Var::X,
            "y" => Var::Y,
            "z" => Var::Z,
            _   => unreachable!()
        }
    }

    fn index(&self) -> usize
    {
        match self
        {
            Var::W => 0,
            Var::X => 1,
            Var::Y => 2,
            Var::Z => 3
        }
    }
}

impl Arg
{
    fn parse(s : &str) -> Arg
    {
        match s.parse()
        {
            Ok(k)  => Arg::Int(k),
            Err(_) => Arg::Var(Var::parse(s))
        }
    }

    fn eval(&self, vars : &[i64 ; 4]) -> i64
    {
        match self
        {
            Arg::Int(k) => *k,
            Arg::Var(v) => vars[v.index()]
        }
    }
}

fn search(prog : &[Inst], mut pc : usize, mut vars : [i64 ; 4], it : impl Iterator<Item = i64> + Clone, cache : &mut HashMap<([i64 ; 4], usize), Option<i64>>) -> Option<i64>
{
    while let Some(inst) = prog.get(pc)
    {
        match inst
        {
            Inst::Add(a, b) => vars[a.index()] += b.eval(&vars),
            Inst::Mul(a, b) => vars[a.index()] *= b.eval(&vars),
            Inst::Div(a, b) => vars[a.index()] /= b.eval(&vars),
            Inst::Mod(a, b) => vars[a.index()] %= b.eval(&vars),
            Inst::Eql(a, b) => vars[a.index()]  = (vars[a.index()] == b.eval(&vars)) as i64,
            Inst::Inp(a)    =>
            {
                if let Some(&digits) = cache.get(&(vars, pc)) { return digits }

                let digits = it.clone().filter_map(|k|
                {
                    let mut vars    = vars;
                    vars[a.index()] = k;
                    search(prog, pc+1, vars, it.clone(), cache).map(|digits| 10*digits + k)
                })
                .next();

                cache.insert((vars, pc), digits);
                return digits
            }
        }

        pc += 1;
    }

    (vars[Var::Z.index()] == 0).then_some(0)
}

fn reverse_digits(mut k : i64) -> i64
{
    let mut rev = 0;
    while k > 0
    {
        rev = 10 * rev + k % 10;
        k  /= 10;
    }
    rev
}
