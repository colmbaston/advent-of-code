use std::collections::{ HashMap, hash_map::Entry };

fn main()
{
    let signal   = include_str!("../input.txt").trim_end().as_bytes();
    let mut hist = HashMap::new();
    for size in [4, 14]
    {
        let (init, rest) = signal.split_at(size);
        hist.clear();
        for &v in init { *hist.entry(v).or_insert(0) += 1 }

        for ((&exiting, &entering), ix) in signal.iter().zip(rest).zip(size ..)
        {
            if hist.len() == size { println!("{ix}"); break }

            *hist.entry(entering).or_insert(0) += 1;

            if let Entry::Occupied(mut e) = hist.entry(exiting)
            {
                let v = e.get_mut();
                if *v == 1 { e.remove(); } else { *v -= 1 }
            }
        }
    }
}
