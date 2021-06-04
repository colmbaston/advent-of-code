use std::collections::HashMap;

fn main()
{
    // the lexicographic ordering of the strings is the chronological ordering
    let mut input = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    input.sort_unstable();
    let mut input = input.into_iter().map(|s| parse_record(s)).peekable();

    // a mapping of guard ids to the frequency they are asleep per minute
    let mut sleep_logs = HashMap::new();

    // each section of the input begins with a guard
    while let Some(Record::Guard(guard_id)) = input.next()
    {
        // each guard will be followed by an even number of toggles
        while input.peek().map_or(false, |r| matches!(r, Record::Toggle(_)))
        {
            // the first of each pair of toggles denotes the minute the guard
            // falls asleep and the second denotes the minute the guard wakes up
            let pair = input.next().and_then(|r| input.next().map(|q| (r, q)));
            if let Some((Record::Toggle(asleep), Record::Toggle(awake))) = pair
            {
                // for each minute, count the frequency the guard is asleep
                let h = sleep_logs.entry(guard_id).or_insert([0 ; 60]);
                for minute in asleep .. awake { h[minute as usize] += 1 }
            }
        }
    }

    // part 1: choose the guard who is asleep the most and the minute they are asleep the most
    let (chosen_guard, log) = sleep_logs.iter().max_by_key(|(_, log)| log.iter().sum::<u32>()).unwrap();
    let  chosen_minute      = log.iter().enumerate().max_by_key(|&(_, k)| k).unwrap().0;
    println!("{}", chosen_guard * chosen_minute as u32);

    // part 2: choose the guard who is most frequently asleep on the same minute, and choose that minute
    let (chosen_guard, (chosen_minute, _)) = sleep_logs.iter().map(|(guard, log)|
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

fn parse_record(s : &str) -> Record
{
    fn span_digits(s : &str) -> (&str, &str)
    {
        s.split_at(s.find(|c : char| !c.is_ascii_digit()).unwrap_or_else(|| s.len()))
    }

    let (minute, s) = span_digits(&s[15..]);

    match s.as_bytes()[2]
    {
        b'G' =>
        {
            let (guard_id, _) = span_digits(&s[9..]);
            Record::Guard(guard_id.parse().unwrap())
        },
        _ => Record::Toggle(minute.parse().unwrap())
    }
}
