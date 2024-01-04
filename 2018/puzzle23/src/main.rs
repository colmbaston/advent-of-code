use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main()
{
    let bots = include_str!("../input.txt").lines().map(Bot::parse).collect::<Vec<Bot>>();

    // part 1: how many bots are in range of the bot with the largest signal radius?
    let largest = bots.iter().max_by_key(|b| b.radius).unwrap();
    println!("{}", bots.iter().filter(|b| largest.contains(b.position)).count());

    // find the bounding rectangle of the bots radii to construct the initial OctreeNode
    let min_corner = (i32::MIN, i32::MIN, i32::MIN);
    let max_corner = (i32::MAX, i32::MAX, i32::MAX);
    let (min_bound, max_bound) = bots.iter().fold((max_corner, min_corner), |((min_x, min_y, min_z), (max_x, max_y, max_z)), b|
    {
        let (x, y, z) = b.min_bound();
        let min_bound = (min_x.min(x), min_y.min(y), min_z.min(z));

        let (x, y, z) = b.max_bound();
        let max_bound = (max_x.max(x), max_y.max(y), max_z.max(z));

        (min_bound, max_bound)
    });

    // compute the priority information that will be stored in the queue
    let priority = |node : OctreeNode|
    {
        let bot_count = bots.iter().filter(|b| node.in_range(b)).count();
        let distance  = manhattan((0, 0, 0), node.min_corner);

        (bot_count, Reverse(node.size), Reverse(distance), node)
    };

    // part 2: priority search for a OctreeNode of size 1 that maximises the
    // number of bots in range. Print the manhattan distance to the origin.
    let mut queue = BinaryHeap::new();
    queue.push(priority(OctreeNode::new(min_bound, max_bound)));
    while let Some((_, Reverse(size), Reverse(distance), node)) = queue.pop()
    {
        if size == 1
        {
            println!("{}", distance);
            break
        }

        queue.extend(node.split().map(priority));
    }
}

type Pos = (i32, i32, i32);

fn manhattan((x1, y1, z1) : Pos, (x2, y2, z2) : Pos) -> u32
{
    ((x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()) as u32
}

struct Bot
{
    position: Pos,
    radius:   u32
}

impl Bot
{
    fn parse(s : &str) -> Bot
    {
        fn parse_integer(s : &str) -> (i32, &str)
        {
            let (integer, rest) = s.split_at(s.find(|c : char| !(c.is_ascii_digit() || c == '-')).unwrap_or(s.len()));
            (integer.parse().unwrap(), rest)
        }

        let (x, s) = parse_integer(&s[5..]);
        let (y, s) = parse_integer(&s[1..]);
        let (z, s) = parse_integer(&s[1..]);
        let (r, _) = parse_integer(&s[5..]);

        Bot { position: (x, y, z), radius: r as u32 }
    }

    fn contains(&self, position : Pos) -> bool
    {
        manhattan(position, self.position) <= self.radius
    }

    fn min_bound(&self) -> Pos
    {
        let (x, y, z) = self.position;
        let r         = self.radius as i32;

        (x - r, y - r, z - r)
    }

    fn max_bound(&self) -> Pos
    {
        let (x, y, z) = self.position;
        let r         = self.radius as i32;

        (x + r, y + r, z + r)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct OctreeNode
{
    size:       u32,
    min_corner: Pos
}

impl OctreeNode
{
    fn new((min_x, min_y, min_z) : Pos, (max_x, max_y, max_z) : Pos) -> OctreeNode
    {
        // find the largest side-length of the bounding rectangle
        let side_length = 1 + (max_x - min_x).max(max_y - min_y).max(max_z - min_z) as u32;

        // the size is the smallest power of two which is at least side_length
        OctreeNode
        {
            size:       (0 ..).map(|n| 2_u32.pow(n)).find(|&s| s >= side_length).unwrap(),
            min_corner: (min_x, min_y, min_z)
        }
    }

    fn max_corner(&self) -> Pos
    {
        let (x, y, z) = self.min_corner;
        let offset    = self.size as i32 - 1;

        (x + offset, y + offset, z + offset)
    }

    // a node is in range of a bot if the manhattan distance from
    // the bot to any of the node;s edges is at most the bot's radius
    fn in_range(&self, bot : &Bot) -> bool
    {
        let (min_x, min_y, min_z) = self.min_corner;
        let (max_x, max_y, max_z) = self.max_corner();
        let (bot_x, bot_y, bot_z) = bot.position;

        let mut distance = 0;
        if bot_x < min_x { distance += min_x - bot_x }
        if bot_x > max_x { distance += bot_x - max_x }
        if bot_y < min_y { distance += min_y - bot_y }
        if bot_y > max_y { distance += bot_y - max_y }
        if bot_z < min_z { distance += min_z - bot_z }
        if bot_z > max_z { distance += bot_z - max_z }
        distance as u32 <= bot.radius
    }

    // split this node into eight smaller nodes by subdividing each axis by two
    fn split(&self) -> impl Iterator<Item = OctreeNode>
    {
        let size      = self.size / 2;
        let s         = size as i32;
        let (x, y, z) = self.min_corner;

        vec![(x,   y,   z  ),
             (x,   y,   z+s),
             (x,   y+s, z  ),
             (x,   y+s, z+s),
             (x+s, y,   z  ),
             (x+s, y,   z+s),
             (x+s, y+s, z  ),
             (x+s, y+s, z+s)].into_iter()
                             .map(move |min_corner| OctreeNode { size, min_corner })
    }
}
