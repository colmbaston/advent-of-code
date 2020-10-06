use std::collections::VecDeque;

fn main()
{
    let digits = include_str!("../input.txt").trim_end().bytes().map(|b| b - b'0').collect::<Vec<u8>>();
    let number = digits.iter().fold(0, |a, &x| 10*a + x as usize);

    // initialise capacity to number+11 in case the final step gives two recipes
    let mut state = State
    {
        elf_one:    0,
        elf_two:    1,
        scoreboard: vec![3, 7],
        iter_index: 0
    };

    // part 1: print the scores of the first 10 recipes immediately after number
    state.iter().skip(number).take(10).for_each(|b| print!("{}", b));
    println!();

    // part 2: reset the iterator and initialise a sliding
    // window to compare with the input digits at each step
    state.iter_index = digits.len();
    let mut window   = state.scoreboard[.. digits.len()].iter().copied().collect::<VecDeque<u8>>();
    for (i, b) in state.iter().enumerate()
    {
        // if the current window matches the input digits, print the index and terminate
        if digits.iter().zip(window.iter()).all(|(x, y)| x == y)
        {
            println!("{}", i);
            break
        }

        // otherwise, slide the window to the right one step
        window.pop_front();
        window.push_back(b);
    }
}

struct State
{
    elf_one    : usize,
    elf_two    : usize,
    scoreboard : Vec<u8>,
    iter_index : usize
}

impl State
{
    fn generate(&mut self)
    {
        let r_one = self.scoreboard[self.elf_one];
        let r_two = self.scoreboard[self.elf_two];

        let sum = r_one + r_two;
        if sum >= 10 { self.scoreboard.push(1) }
        self.scoreboard.push(sum % 10);

        self.elf_one = (self.elf_one + 1 + r_one as usize) % self.scoreboard.len();
        self.elf_two = (self.elf_two + 1 + r_two as usize) % self.scoreboard.len();
    }

    fn iter(&mut self) -> &mut State
    {
        self
    }
}

impl Iterator for State
{
    type Item = u8;

    fn next(&mut self) -> Option<u8>
    {
        while self.scoreboard.len() <= self.iter_index
        {
            self.generate()
        }

        let result = self.scoreboard[self.iter_index];
        self.iter_index += 1;
        Some(result)
    }
}
