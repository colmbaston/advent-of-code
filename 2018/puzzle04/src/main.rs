use std::collections::{ HashMap, VecDeque };

fn main()
{
    // the lexicographic ordering of the strings is the chronological ordering
    let mut input = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    input.sort_unstable();
    let mut input = input.into_iter().map(|s| parse_record(s).unwrap().1).collect::<VecDeque<Record>>();

    // each section of the input begins with a guard
    let mut histograms = HashMap::new();
    while let Some(Record::Guard(guard_id)) = input.pop_front()
    {
        // each guard will have an even number of toggles
        while input.front().map_or(false, |r| r.is_toggle())
        {
            // the first of each pair of toggles denotes the minute the guard
            // falls asleep and the second denotes the minute the guard wakes up
            let pair = (input.pop_front(), input.pop_front());
            if let (Some(Record::Toggle(asleep)), Some(Record::Toggle(awake))) = pair
            {
                // for each minute, count the frequency the guard is asleep
                let h = histograms.entry(guard_id).or_insert([0 ; 60]);
                for minute in asleep .. awake { h[minute as usize] += 1 }

            }
        }
    }

    // part 1: choose the guard who is asleep the most and the minute they are asleep the most
    let (chosen_guard, log) = histograms.iter().max_by_key(|(_, log)| log.iter().sum::<u32>()).unwrap();
    let  chosen_minute      = log.iter().enumerate().max_by_key(|&(_, k)| k).unwrap().0;
    println!("{}", chosen_guard * chosen_minute as u32);

    // part 2: choose the guard who is most frequently asleep on the same minute, and choose that minute
    let (chosen_guard, (chosen_minute, _)) = histograms.iter().map(|(guard, log)|
    {
        (guard, log.iter()
                   .enumerate()
                   .max_by_key(|&(_, k)| k)
                   .unwrap())
    })
    .max_by_key(|&(_, (_, k))| k)
    .unwrap();
    println!("{}", chosen_guard * chosen_minute as u32);
}

enum Record
{
    Guard(u32),
    Toggle(u32)
}

impl Record
{
    fn is_toggle(&self) -> bool
    {
        match self
        {
            Record::Toggle(_) => true,
            _                 => false
        }
    }
}

fn parse_record(s : &str) -> nom::IResult<&str, Record>
{
    use nom::character::complete::digit1;

    let (s, minute) = digit1(&s[15..])?;

    let record = match s.as_bytes()[2]
    {
        b'G' =>
        {
            let (_, guard_id) = digit1(&s[9..])?;
            Record::Guard(guard_id.parse().unwrap())
        },
        _ => Record::Toggle(minute.parse().unwrap()),
    };

    Ok(("", record))
}
