
pub const DIGITS : usize = 6;

#[derive(Clone)]
pub struct NonDescending
{
    pub digits : [u8 ; DIGITS]
}

impl Iterator for NonDescending
{
    type Item = [u8 ; DIGITS];

    fn next(&mut self) -> Option<[u8 ; DIGITS]>
    {
        let result = Some(self.digits);

        for x in self.digits.iter_mut().rev()
        {
            *x = (*x + 1) % 10;
            if *x != 0 { break }
        }

        for i in 0 .. DIGITS-1
        {
            if self.digits[i+1] < self.digits[i]
            {
                self.digits[i+1] = self.digits[i]
            }
        }

        result
    }
}
