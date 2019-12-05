pub mod intcode
{
    pub fn parse_file(fp : &str) -> Vec<i64>
    {
        std::fs::read_to_string(fp).unwrap().trim_end().split(',').map(|x| x.parse().unwrap()).collect()
    }

    pub fn interpret(memory : &mut [i64])
    {
        let mut ip = 0;
        loop
        {
            let inc = match memory.get(ip)
            {
                None     => break,
                Some(1)  => { intcode_binop(|x, y| x + y, &ip, memory); 4 }
                Some(2)  => { intcode_binop(|x, y| x * y, &ip, memory); 4 }
                Some(99) => break,
                Some(k)  => panic!("unimplemented opcode: {}", k)
            };
            ip += inc
        }
    }

    fn intcode_binop<F : Fn(i64, i64) -> i64>(op : F, ip : &usize, memory : &mut [i64])
    {
        memory[memory[*ip+3] as usize] = op(memory[memory[*ip+1] as usize], memory[memory[*ip+2] as usize]);
    }
}
