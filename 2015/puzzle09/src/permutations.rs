pub struct Permutations<T>
{
    data:   Option<Vec<T>>,
    output: Option<Vec<T>>
}

impl<T> Permutations<T>
{
    pub fn new(values : impl Iterator<Item = T>) -> Permutations<T>
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
