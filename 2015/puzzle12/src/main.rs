use json::JsonValue;

fn main()
{
    let input = json::parse(include_str!("../input.txt")).unwrap();

    println!("{}", sum_json(&input, |_| true));
    println!("{}", sum_json(&input, |j| match j
    {
        JsonValue::Object(o) => o.iter().all(|(_, v)| if let JsonValue::Short(s) = v { s != "red" } else { true }),
        _                    => true
    }));
}

fn sum_json(j : &JsonValue, p : impl Copy + Fn(&JsonValue) -> bool) -> i32
{
    if !p(j) { return 0 }

    match j
    {
        JsonValue::Number(k) => { let k : f64 = (*k).into(); k as i32 },
        JsonValue::Array(v)  => v.iter().fold(0, |k, x| k + sum_json(x,   p)),
        JsonValue::Object(o) => o.iter().fold(0, |k, x| k + sum_json(x.1, p)),
        _                    => 0
    }
}
