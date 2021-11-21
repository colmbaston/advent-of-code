fn main()
{
    let mut input = include_str!("../input.txt").lines().map(|s| s.split(": ").skip(1).next().unwrap().parse::<i32>().unwrap());
    let boss_hp   = input.next().unwrap();
    let boss_d    = input.next().unwrap();
    let boss_a    = input.next().unwrap();

    let win = |&(_, d, a) : &(i32, i32, i32)|
    {
        let hit      = (d - boss_a).max(1);
        let boss_hit = (boss_d - a).max(1);

        (boss_hp + hit - 1) / hit <= (100 + boss_hit - 1) / boss_hit
    };

    let mut equipment = select_equipment();
    equipment.sort_unstable();
    println!("{}", equipment.iter().      find(|&e|  win(e)).unwrap().0);
    println!("{}", equipment.iter().rev().find(|&e| !win(e)).unwrap().0);
}

fn select_equipment() -> Vec<(i32, i32, i32)>
{
    select_weapon().into_iter()
                   .flat_map(move |(wc, wd, wa)|
                      select_armour().into_iter()
                                     .flat_map(move |(ac, ad, aa)|
                                        select_rings().into_iter()
                                                      .map(move |(rc, rd, ra)| (wc+ac+rc, wd+ad+rd, wa+aa+ra)))).collect()
}

fn select_weapon() -> Vec<(i32, i32, i32)>
{
    vec![( 8, 4, 0),
         (10, 5, 0),
         (25, 6, 0),
         (40, 7, 0),
         (74, 8, 0)]
}

fn select_armour() -> Vec<(i32, i32, i32)>
{
    vec![(  0, 0, 0),
         ( 13, 0, 1),
         ( 31, 0, 2),
         ( 53, 0, 3),
         ( 75, 0, 4),
         (102, 0, 5)]
}

fn select_rings() -> Vec<(i32, i32, i32)>
{
    let rings = [( 25, 1, 0),
                 ( 50, 2, 0),
                 (100, 3, 0),
                 ( 20, 0, 1),
                 ( 40, 0, 2),
                 ( 80, 0, 3)];

    let mut v = vec![(0, 0, 0)];
    v.extend(rings.iter().cloned());

    for (i, (lc, ld, la)) in rings.iter().enumerate()
    {
        for (rc, rd, ra) in rings.iter().skip(i+1)
        {
            v.push((lc+rc, ld+rd, la+ra));
        }
    }

    v
}
