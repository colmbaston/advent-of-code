use std::collections::HashMap;

fn main()
{
    let mut input = include_str!("../input.txt").lines().map(parse_op).collect::<HashMap<&str, Op>>();
    let mut cache = HashMap::new();

    let a = eval("a", &input, &mut cache);
    println!("{}", a);

    cache.clear();
    input.insert("b", Op::Atom(Atom::Const(a)));
    println!("{}", eval("a", &input, &mut cache));
}

enum Op<'a>
{
    Atom(Atom<'a>),
    And(Atom<'a>, Atom<'a>),
    Or(Atom<'a>, Atom<'a>),
    Not(Atom<'a>),
    LShift(Atom<'a>, Atom<'a>),
    RShift(Atom<'a>, Atom<'a>)
}

enum Atom<'a>
{
    Const(u16),
    Var(&'a str)
}

fn parse_op(s : &str) -> (&str, Op<'_>)
{
    match s.split(' ').collect::<Vec<&str>>()[..]
    {
        [a,              "->", c] => (c, Op::Atom(parse_atom(a))),
        [a, "AND",    b, "->", c] => (c, Op::And(parse_atom(a), parse_atom(b))),
        [a, "OR",     b, "->", c] => (c, Op::Or(parse_atom(a), parse_atom(b))),
        [   "NOT",    b, "->", c] => (c, Op::Not(parse_atom(b))),
        [a, "LSHIFT", b, "->", c] => (c, Op::LShift(parse_atom(a), parse_atom(b))),
        [a, "RSHIFT", b, "->", c] => (c, Op::RShift(parse_atom(a), parse_atom(b))),
        _                         => unreachable!()
    }
}

fn parse_atom(s : &str) -> Atom<'_>
{
    match s.parse()
    {
        Ok(k)  => Atom::Const(k),
        Err(_) => Atom::Var(s)
    }
}

fn eval<'a>(s : &'a str, m : &HashMap<&'a str, Op<'a>>, cache : &mut HashMap<&'a str, u16>) -> u16
{
    if let Some(&k) = cache.get(s) { return k }

    let k = match m.get(s).unwrap()
    {
        Op::Atom(a)      =>  eval_atom(a, m, cache),
        Op::And(a, b)    =>  eval_atom(a, m, cache) &  eval_atom(b, m, cache),
        Op::Or(a, b)     =>  eval_atom(a, m, cache) |  eval_atom(b, m, cache),
        Op::Not(a)       => !eval_atom(a, m, cache),
        Op::LShift(a, b) =>  eval_atom(a, m, cache) << eval_atom(b, m, cache),
        Op::RShift(a, b) =>  eval_atom(a, m, cache) >> eval_atom(b, m, cache)
    };
    cache.insert(s, k);

    k
}

fn eval_atom<'a>(a : &Atom<'a>, m : &HashMap<&'a str, Op<'a>>, cache : &mut HashMap<&'a str, u16>) -> u16
{
    match *a
    {
        Atom::Const(k) => k,
        Atom::Var(v)   => eval(v, m, cache)
    }
}
