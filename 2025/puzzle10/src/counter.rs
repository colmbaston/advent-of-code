pub struct Counter
{
    values:   Vec<i32>,
    sum:      i32,
    bound:    i32,
    finished: bool
}

impl Counter
{
    pub fn new(size : usize, bound : i32) -> Counter
    {
        Counter { values: vec![0 ; size], sum: 0, bound, finished: bound < 0 }
    }
}

impl Iterator for Counter
{
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Vec<i32>>
    {
        if self.finished { return None }
        let output = self.values.clone();

        for v in self.values.iter_mut()
        {
            *v += 1;
            self.sum += 1;
            if self.sum <= self.bound
            {
                return Some(output)
            }
            else
            {
                self.sum -= *v;
                *v = 0;
            }
        }
        self.finished = true;
        Some(output)
    }
}
