fn main()
{
    let input = include_str!("../input.txt").trim_end().bytes().map(|x| (x - b'0') as i64).collect::<Vec<_>>();

    {
        let mut fft_one = input.clone();
        let mut fft_two = input.clone();

        fn base_pattern(x : usize) -> impl Iterator<Item = i64>
        {
            use std::iter::repeat_n;

                   repeat_n( 0, x)
            .chain(repeat_n( 1, x))
            .chain(repeat_n( 0, x))
            .chain(repeat_n(-1, x))
            .cycle().skip(1)
        }

        for _ in 0 .. 100
        {
            for (i, x) in (0..).zip(fft_two.iter_mut())
            {
                let mut digit = 0;
                for (y, b) in fft_one.iter().zip(base_pattern(i+1)).skip(i)
                {
                    digit += y * b;
                }
                *x = digit.abs() % 10;
            }
            std::mem::swap(&mut fft_one, &mut fft_two);
        }

        fft_one[..8].iter().for_each(|x| print!("{}", x));
        println!();
    }

    {
        let message_offset = input[..7].iter().fold(0, |a, &d| a * 10 + d as usize);
        let suffix_len     = input.len() * 10_000 - message_offset;

        if message_offset < suffix_len
        {
            panic!("message offset is assumed to be in second half of signal");
        }

        let mut signal_suffix : Vec<i64> = input.iter().cycle().skip(message_offset).take(suffix_len).copied().collect();

        for _ in 0 .. 100
        {
            let mut last = signal_suffix[suffix_len - 1];
            for x in signal_suffix.iter_mut().rev().skip(1)
            {
                *x = (*x + last) % 10;
                last = *x;
            }
        }

        signal_suffix[..8].iter().for_each(|x| print!("{}", x));
        println!();
    }
}
