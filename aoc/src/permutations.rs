pub struct Permutations<T>
{
    data: Option<Vec<T>>
}

impl<T : Ord> Permutations<T>
{
    pub fn from_sorted(values : impl Iterator<Item = T>) -> Permutations<T>
    {
        Permutations { data: Some(values.collect()) }
    }

    pub fn from_unsorted(values : impl Iterator<Item = T>) -> Permutations<T>
    {
        let mut data = values.collect::<Vec<T>>();
        data.sort_unstable();
        Permutations { data: Some(data) }
    }
}

impl<T : Clone + Ord> Iterator for Permutations<T>
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>>
    {
        let output = self.data.clone();

        if let Some(data) = &mut self.data
        {
            match data.windows(2).enumerate().rev().find(|(_, w)| w[0] < w[1])
            {
                None         => self.data = None,
                Some((k, _)) =>
                {
                    let data_k = &data[k];
                    let (l, _) = data.iter().enumerate().rev().find(|(_, x)| data_k < x).unwrap();
                    data.swap(k, l);
                    data[k+1 ..].reverse();
                }
            }
        }

        output
    }
}
