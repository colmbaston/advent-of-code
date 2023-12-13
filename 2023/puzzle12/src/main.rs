use std::collections::HashMap;

fn main()
{
    let mut records = include_str!("../input.txt").lines().map(Record::parse).collect::<Vec<Record>>();
    println!("{}", records.iter().map(|r| r.arrangements(0, 0, &mut HashMap::new())).sum::<u64>());
    records.iter_mut().for_each(Record::unfold);
    println!("{}", records.iter().map(|r| r.arrangements(0, 0, &mut HashMap::new())).sum::<u64>());
}

struct Record
{
    springs: Vec<Spring>,
    damaged: Vec<usize>
}

#[derive(Copy, Clone)]
enum Spring { Operational, Damaged, Unknown }

impl Record
{
    fn parse(s : &str) -> Record
    {
        let (springs, damaged) = s.split_once(' ').unwrap();
        let springs = springs.bytes().map(Spring::parse).collect();
        let damaged = damaged.split(',').map(|k| k.parse::<usize>().unwrap()).collect();
        Record { springs, damaged }
    }

    fn unfold(&mut self)
    {
        self.springs = self.springs.iter().copied()
                                   .chain(std::iter::once(Spring::Unknown))
                                   .cycle()
                                   .take(5 * self.springs.len() + 4)
                                   .collect();
        self.damaged = self.damaged.iter().copied()
                                   .cycle()
                                   .take(5 * self.damaged.len())
                                   .collect();
    }

    fn arrangements(&self, mut springs_ix : usize, damaged_ix : usize, cache : &mut HashMap<(usize, usize), u64>) -> u64
    {
        let springs = &self.springs[springs_ix ..];
        springs_ix += springs.iter()
                             .position(|s| s.damaged())
                             .unwrap_or(springs.len());

        if let Some(&count) = cache.get(&(springs_ix, damaged_ix))
        {
            return count
        }

        let springs = &self.springs[springs_ix ..];
        let damaged = &self.damaged[damaged_ix ..];

        if damaged.is_empty()
        {
            return springs.iter().all(|s| s.operational()) as u64
        }

        if springs.len() < damaged.iter().sum()
        {
            return 0
        }

        let mut count = 0;

        if springs[0].operational()
        {
            count += self.arrangements(springs_ix+1, damaged_ix, cache)
        }

        if springs[0].damaged()
        {
            let len = damaged[0];
            if springs[.. len].iter().all(|s| s.damaged())
            {
                count += match springs.get(len)
                {
                    None                       => 1,
                    Some(s) if s.operational() => self.arrangements(springs_ix+len+1, damaged_ix+1, cache),
                    _                          => 0
                }
            }
        }

        cache.insert((springs_ix, damaged_ix), count);
        count
    }
}

impl Spring
{
    fn parse(b : u8) -> Spring
    {
        match b
        {
            b'.' => Spring::Operational,
            b'#' => Spring::Damaged,
            b'?' => Spring::Unknown,
            _    => unreachable!()
        }
    }

    fn operational(self) -> bool
    {
        matches!(self, Spring::Operational | Spring::Unknown)
    }

    fn damaged(self) -> bool
    {
        matches!(self, Spring::Damaged | Spring::Unknown)
    }
}
