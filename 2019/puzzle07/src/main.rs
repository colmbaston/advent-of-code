use intcode::Interpreter;
use std::sync::mpsc::channel;

fn main()
{
    let input = intcode::parse_file!("../input.txt");
    run(&input, false);
    run(&input, true);
}

fn run(input : &[i64], feedback : bool)
{
    let mut max = std::i64::MIN;
    for perm in Permutations::new(if feedback { 5 .. 10 } else { 0 .. 5 })
    {
        let (first_send, first_recv) = channel();
        let mut last_recv = first_recv;
        let mut last_send = first_send.clone();
        let mut handles   = Vec::new();

        for x in perm.into_iter()
        {
            last_send.send(x).unwrap();
            let (sender, receiver) = channel();
            last_send = sender.clone();
            handles.push(Interpreter::with_channel(input.to_vec(), last_recv, sender, None));
            last_recv = receiver;
        }

        drop(last_send);
        first_send.send(0).unwrap();

        for x in last_recv.iter()
        {
            if max < x { max = x }
            if feedback { let _ = first_send.send(x); }
        }

        while let Some(h) = handles.pop()
        {
            h.join().unwrap();
        }
    }
    println!("{}", max);
}

struct Permutations<T>
{
    data:   Option<Vec<T>>,
    output: Option<Vec<T>>
}

impl<T> Permutations<T>
{
    fn new(values : impl Iterator<Item = T>) -> Permutations<T>
    {
        let data = Some(values.collect());
        Permutations { data, output: None }
    }
}

impl<T : Clone + Ord> Iterator for Permutations<T>
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>>
    {
        if let Some(data) = &mut self.data
        {
            self.output = Some(data.clone());

            match data.windows(2).enumerate().rev().find(|(_, w)| w[0] < w[1])
            {
                None         => self.data = None,
                Some((k, _)) =>
                {
                    let data_k = &data[k];
                    let (l, _) = data.iter().enumerate().rev().find(|(_, x)| data_k < x).unwrap();
                    data.swap(k, l);
                    data[k+1..].reverse();
                }
            }
        }

        self.output.take()
    }
}
