mod packet;
use packet::Packet;

fn main()
{
    let mut packets = include_str!("../input.txt").lines()
                                                  .filter_map(Packet::parse_all)
                                                  .collect::<Vec<Packet>>();

    println!("{}", packets.array_windows().step_by(2).zip(1 ..)
                          .filter_map(|([a, b], ix)| (a <= b).then_some(ix))
                          .sum::<u32>());

    let dividers = ["[[2]]", "[[6]]"].into_iter()
                                     .filter_map(Packet::parse_all)
                                     .collect::<Vec<Packet>>();

    packets.extend(dividers.iter().cloned());
    packets.sort_unstable();
    println!("{}", dividers.into_iter()
                           .filter_map(|d| packets.binary_search(&d).ok().map(|ix| ix as u32 + 1))
                           .product::<u32>());
}
