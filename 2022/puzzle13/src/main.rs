mod packet;
use packet::Packet;

fn main()
{
    let input = include_str!("../input.txt").split("\n\n")
                                            .filter_map(|pair|
                                            {
                                                let mut lines = pair.lines();
                                                Some((Packet::parse_all(lines.next()?)?,
                                                      Packet::parse_all(lines.next()?)?))
                                            })
                                            .collect::<Vec<(Packet, Packet)>>();

    for (a, b) in input
    {
        println!("{a:?}");
        println!("{b:?}");
        println!();
    }
}
