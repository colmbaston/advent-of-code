use std::collections::HashMap;

fn main()
{
    let mut input = include_str!("../input.txt").lines().map(parse_op).collect::<HashMap<&str, Op>>();
    let mut cache = HashMap::new();

    let a = eval("a", &input, &mut cache);
    println!("{}", a);

    cache.clear();
    input.insert("b", Op::ATOM(Atom::CONST(a)));
    println!("{}", eval("a", &input, &mut cache));
}

enum Op<'a>
{
    ATOM(Atom<'a>),
    AND(Atom<'a>, Atom<'a>),
    OR(Atom<'a>, Atom<'a>),
    NOT(Atom<'a>),
    LSHIFT(Atom<'a>, Atom<'a>),
    RSHIFT(Atom<'a>, Atom<'a>)
}

enum Atom<'a>
{
    CONST(u16),
    VAR(&'a str)
}

fn parse_op(s : &str) -> (&str, Op)
{
    match s.split(' ').collect::<Vec<&str>>()[..]
    {
        [a,              "->", c] => (c, Op::ATOM(parse_atom(a))),
        [a, "AND",    b, "->", c] => (c, Op::AND(parse_atom(a), parse_atom(b))),
        [a, "OR",     b, "->", c] => (c, Op::OR(parse_atom(a), parse_atom(b))),
        [   "NOT",    b, "->", c] => (c, Op::NOT(parse_atom(b))),
        [a, "LSHIFT", b, "->", c] => (c, Op::LSHIFT(parse_atom(a), parse_atom(b))),
        [a, "RSHIFT", b, "->", c] => (c, Op::RSHIFT(parse_atom(a), parse_atom(b))),
        _                         => unreachable!()
    }
}

fn parse_atom(s : &str) -> Atom
{
    match s.parse()
    {
        Ok(k)  => Atom::CONST(k),
        Err(_) => Atom::VAR(s)
    }
}

fn eval<'a>(s : &'a str, m : &HashMap<&'a str, Op<'a>>, cache : &mut HashMap<&'a str, u16>) -> u16
{
    if let Some(&k) = cache.get(s) { return k }

    let k = match m.get(s).unwrap()
    {
        Op::ATOM(a)      =>  eval_atom(a, m, cache),
        Op::AND(a, b)    =>  eval_atom(a, m, cache) &  eval_atom(b, m, cache),
        Op::OR(a, b)     =>  eval_atom(a, m, cache) |  eval_atom(b, m, cache),
        Op::NOT(a)       => !eval_atom(a, m, cache),
        Op::LSHIFT(a, b) =>  eval_atom(a, m, cache) << eval_atom(b, m, cache),
        Op::RSHIFT(a, b) =>  eval_atom(a, m, cache) >> eval_atom(b, m, cache)
    };
    cache.insert(s, k);

    k
}

fn eval_atom<'a>(a : &Atom<'a>, m : &HashMap<&'a str, Op<'a>>, cache : &mut HashMap<&'a str, u16>) -> u16
{
    match *a
    {
        Atom::CONST(k) => k,
        Atom::VAR(v)   => eval(v, m, cache)
    }
}
