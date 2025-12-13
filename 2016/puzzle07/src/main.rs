fn main()
{
    let ips = include_str!("../input.txt").lines().collect::<Vec<&str>>();

    println!("{}", ips.iter().filter(|&ip| supports_tls(ip)).count());
    println!("{}", ips.iter().filter(|&ip| supports_ssl(ip)).count());
}

fn supports_tls(ip : &str) -> bool
{
    let mut hypernet = 0;
    let mut found    = false;
    ip.as_bytes().array_windows().all(|&[a, b, c, d]|
    {
        match a
        {
            b'[' => { hypernet += 1; true }
            b']' => { hypernet -= 1; true }
            _    => a == b || a != d || b != c || { found = true; hypernet == 0 }
        }
    })
    && found
}

fn supports_ssl(ip : &str) -> bool
{
    let mut hypernet = 0;
    ip.as_bytes().array_windows().any(|&[a, b, c]|
    {
        match a
        {
            b'[' => { hypernet += 1; false }
            b']' => { hypernet -= 1; false }
            _    => hypernet == 0 && a != b && a == c && supports_ssl_aux(ip, a, b)
        }
    })
}

fn supports_ssl_aux(ip : &str, a : u8, b : u8) -> bool
{
    let mut hypernet = 0;
    ip.as_bytes().array_windows().any(|&[c, d, e]|
    {
        match c
        {
            b'[' => { hypernet += 1; false },
            b']' => { hypernet -= 1; false },
            _    => hypernet > 0 && c == b && d == a && e == b
        }
    })
}
