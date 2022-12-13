#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
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
        s.strip_prefix('[').and_then(|tail|
        {
            let mut payloads = Vec::new();
            tail.strip_prefix(']').or_else(||
            {
                let mut parse = Payload::parse(tail)?;
                payloads.push(parse.0);
                while let Some(rest) = parse.1.strip_prefix(',')
                {
                    parse = Payload::parse(rest)?;
                    payloads.push(parse.0);
                }
                parse.1.strip_prefix(']')
            })
            .map(|rest| (Packet(payloads), rest))
        })
    }

    fn as_slice(&self) -> &[Payload]
    {
        self.0.as_slice()
    }
}

impl Payload
{
    fn parse(s : &str) -> Option<(Payload, &str)>
    {
        s.as_bytes().first().and_then(|b| match b
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

impl Ord for Payload
{
    fn cmp(&self, other : &Payload) -> Ordering
    {
        match (self, other)
        {
            (  Payload::Int(k),      Payload::Int(l))    => k.cmp(l),
            (k@Payload::Int(_),      Payload::Packet(q)) => [k.clone()].as_slice().cmp(q.as_slice()),
            (  Payload::Packet(p), l@Payload::Int(_))    => p.as_slice().cmp([l.clone()].as_slice()),
            (  Payload::Packet(p),   Payload::Packet(q)) => p.cmp(q)
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
