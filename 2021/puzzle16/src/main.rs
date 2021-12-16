fn main()
{
    let input  = hex_to_bin(include_str!("../input.txt").trim_end());
    let packet = decode_packet(&mut input.as_slice());

    println!("{}", packet.version_sum());
    println!("{}", packet.evaluate());
}

fn hex_to_bin(s : &str) -> Vec<bool>
{
    let mut bin = Vec::with_capacity(s.len() * 4);

    for b in s.bytes().rev()
    {
        let mut d = match b
        {
            b'0' ..= b'9' => b - b'0',
            b'A' ..= b'F' => b - b'A' + 10,
            _             => unreachable!()
        };

        for _ in 0 .. 4
        {
            bin.push(d % 2 == 1);
            d /= 2;
        }
    }

    bin.reverse();
    bin
}

struct Packet
{
    version: u8,
    payload: Payload
}

enum Payload
{
    Literal(u64),
    Operator(Op, Vec<Packet>)
}

enum Op
{
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo
}

impl Packet
{
    fn version_sum(&self) -> u64
    {
        self.version as u64 + match &self.payload
        {
            Payload::Literal(_)      => 0,
            Payload::Operator(_, ps) => ps.iter().map(Packet::version_sum).sum()
        }
    }

    fn evaluate(&self) -> u64
    {
        match &self.payload
        {
            Payload::Literal(lit)     => *lit,
            Payload::Operator(op, ps) =>
            {
                let mut i = ps.iter().map(Packet::evaluate);

                match op
                {
                    Op::Sum         =>  i.sum(),
                    Op::Product     =>  i.product(),
                    Op::Minimum     =>  i.min().unwrap(),
                    Op::Maximum     =>  i.max().unwrap(),
                    Op::GreaterThan => (i.next().unwrap() >  i.next().unwrap()) as u64,
                    Op::LessThan    => (i.next().unwrap() <  i.next().unwrap()) as u64,
                    Op::EqualTo     => (i.next().unwrap() == i.next().unwrap()) as u64
                }
            }
        }
    }
}

fn decode_packet(bin : &mut &[bool]) -> Packet
{
    let version = decode_number(&bin[0 .. 3]) as u8;
    let payload = match decode_number(&bin[3 .. 6])
    {
        4 =>
        {
            let mut lit  = 0;
            let mut more = true;
            *bin         = &bin[6 ..];

            while more
            {
                lit  = 16 * lit + decode_number(&bin[1 .. 5]) as u64;
                more = bin[0];
                *bin = &bin[5 ..];
            }

            Payload::Literal(lit)
        }
        opcode =>
        {
            let mut packets = Vec::new();

            if bin[6]
            {
                let count = decode_number(&bin[7 .. 18]);
                *bin      = &bin[18 ..];

                for _ in 0 .. count
                {
                    packets.push(decode_packet(bin));
                }
            }
            else
            {
                let target = bin.len() - decode_number(&bin[7 .. 22]) as usize - 22;
                *bin       = &bin[22 ..];

                while bin.len() > target
                {
                    packets.push(decode_packet(bin));
                }
            }

            let op = match opcode
            {
                0 => Op::Sum,
                1 => Op::Product,
                2 => Op::Minimum,
                3 => Op::Maximum,
                5 => Op::GreaterThan,
                6 => Op::LessThan,
                7 => Op::EqualTo,
                _ => unreachable!()
            };

            Payload::Operator(op, packets)
        }
    };

    Packet { version, payload }
}

fn decode_number(bin : &[bool]) -> u16
{
    bin.iter().fold(0, |a, &b| 2 * a + b as u16)
}
