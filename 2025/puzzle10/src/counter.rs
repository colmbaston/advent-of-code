pub struct Counter
{
    values:   Vec<i32>,
    max:      i32,
    finished: bool
}

impl Counter
{
    pub fn new(size : usize, max : i32) -> Counter
    {
        Counter { values: vec![0 ; size], max, finished: false }
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
            if *v <= self.max
            {
                return Some(output)
            }
            else
            {
                *v = 0
            }
        }
        self.finished = true;
        Some(output)
    }
}
