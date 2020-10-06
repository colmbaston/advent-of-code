fn main()
{
    let digits = include_str!("../input.txt").trim_end().bytes().map(|b| b - b'0').collect::<Vec<u8>>();
    let number = digits.iter().fold(0, |a, &x| 10*a + x as usize);

    // initialise capacity to number+11 in case the final step gives two recipes
    let mut state = State { elf_one: 0, elf_two: 1, scoreboard: Vec::with_capacity(number+11) };
    state.scoreboard.extend(&[3, 7]);

    // part 1: generate (at least) number+10 recipes
    // and print the scores of the first 10 after number
    while state.scoreboard.len() < number+10 { state.generate() }
    state.scoreboard[number .. number+10].iter().for_each(|b| print!("{}", b));
    println!();

    // part 2: check if the windows of the scoreboard match the input digits; if no
    // match is found, generate more recipes and recheck the newly-generated subslice
    let mut prev_checked = 0;
    'outer: loop
    {
        for (i, w) in state.scoreboard[prev_checked..].windows(digits.len()).enumerate()
        {
            // check if this i
            if w == digits.as_slice()
            {
                println!("{}", prev_checked+i);
                break 'outer
            }
        }

        // update the number of indices that have previously been checked
        prev_checked = state.scoreboard.len() - digits.len() + 1;

        // generate recipes until the new length is double the old capacity
        state.scoreboard.reserve(state.scoreboard.capacity());
        while state.scoreboard.capacity() > state.scoreboard.len()+1
        {
            state.generate();
        }
    }
}

struct State
{
    elf_one    : usize,
    elf_two    : usize,
    scoreboard : Vec<u8>
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
}
