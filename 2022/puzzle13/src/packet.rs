#[derive(Debug)]
pub struct Packet(Vec<Payload>);

#[derive(Debug)]
pub enum Payload
{
    Int(u8),
    Packet(Packet)
}

impl Packet
{
    pub fn parse_all(s : &str) -> Option<Packet>
    {
        Packet::parse(s).and_then(|(p, s)| s.is_empty().then_some(p))
    }

    fn parse(s : &str) -> Option<(Packet, &str)>
    {
        let mut payloads = Vec::new();
        s.strip_prefix('[').and_then(|tail| Packet::parse_payloads(tail, &mut payloads).map(|rest| (Packet(payloads), rest)))
    }

    fn parse_payloads<'a>(s : &'a str, payloads : &mut Vec<Payload>) -> Option<&'a str>
    {
        s.strip_prefix(']').or_else(|| Payload::parse(s).and_then(|(payload, rest)|
        {
            payloads.push(payload);
            Packet::parse_payloads(rest.strip_prefix(',').unwrap_or(rest), payloads)
        }))
    }
}

impl Payload
{
    fn parse(s : &str) -> Option<(Payload, &str)>
    {
        s.as_bytes().get(0).and_then(|b| match b
        {
            b'0' ..= b'9' =>
            {
                let (digits, rest) = s.split_at(s.find(|c : char| !c.is_ascii_digit()).unwrap_or(s.len()));
                digits.parse::<u8>().ok().map(|k| (Payload::Int(k), rest))
            },

            b'[' => Packet::parse(s).map(|(p, rest)| (Payload::Packet(p), rest)),
            _    => None
        })
    }
}
