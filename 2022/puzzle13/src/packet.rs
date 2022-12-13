#[derive(Clone, PartialEq, Eq)]
pub struct Packet(Vec<Payload>);

#[derive(Clone, PartialEq, Eq)]
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

use std::cmp::Ordering;

impl Ord for Packet
{
    fn cmp(&self, other : &Packet) -> Ordering
    {
        for (p, q) in self.0.iter().zip(other.0.iter())
        {
            match p.cmp(q)
            {
                Ordering::Equal => continue,
                ord             => return ord
            }
        }

        self.0.len().cmp(&other.0.len())
    }
}

impl PartialOrd for Packet
{
    fn partial_cmp(&self, other : &Packet) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

impl Ord for Payload
{
    fn cmp(&self, other : &Payload) -> Ordering
    {
        match (self, other)
        {
            (Payload::Int(k),    Payload::Int(l))    => k.cmp(l),
            (k@Payload::Int(_),  Payload::Packet(q)) => Packet(vec![k.clone()]).cmp(q),
            (Payload::Packet(p), l@Payload::Int(_))  => p.cmp(&Packet(vec![l.clone()])),
            (Payload::Packet(p), Payload::Packet(q)) => p.cmp(q)
        }
    }
}

impl PartialOrd for Payload
{
    fn partial_cmp(&self, other : &Payload) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}
