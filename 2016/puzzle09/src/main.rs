fn main()
{
    let input    = include_str!("../input.txt").trim_end();
    let mut freq = vec![1 ; input.len()];

    let mut skip      = 0;
    let mut count_one = 0;
    let mut count_two = 0;
    let mut bytes     = input.bytes().enumerate().peekable();
    while let Some((i, b)) = bytes.next()
    {
        if let b'(' = b
        {
            let mut take_digits = || bytes.by_ref()
                                          .map(|(_, b)| b)
                                          .take_while(|b| b.is_ascii_digit())
                                          .fold(0, |a, k| 10*a + (k - b'0') as u64);

            let len  = take_digits();
            let reps = take_digits();
            let next = bytes.peek().unwrap().0;

            if skip <= i
            {
                count_one += len * reps;
                skip = next + len as usize;
            }
            freq[next .. next + len as usize].iter_mut().for_each(|f| *f *= reps);
        }
        else
        {
            if skip <= i { count_one += 1 }
            count_two += freq[i];
        }
    }

    println!("{count_one}");
    println!("{count_two}");
}
