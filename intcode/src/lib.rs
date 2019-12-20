use std::iter::{ empty, Empty };
use std::io::{ stdin, stdout, Write };
use std::thread::{ spawn, JoinHandle };
use std::sync::mpsc::{ channel, Sender, Receiver };

pub enum Request { Input, Output }

pub struct Interpreter<I>
where I : Iterator<Item = i64>
{
    pub memory : Vec<i64>,
    ip         : usize,
    bp         : usize,
    input_iter : I,
    send_req   : Option<Sender<Request>>
}

#[macro_export]
macro_rules! parse_file
{
    ($file:expr) => { include_str!($file).trim_end().split(',').map(|x| x.parse().unwrap()).collect::<Vec<i64>>() }
}

impl<I : Iterator<Item = i64>> Iterator for &mut Interpreter<I>
{
    type Item = i64;

    fn next(&mut self) -> Option<i64>
    {
        loop
        {
            let (opcode, modes) = match self.memory.get(self.ip)
            {
                None     => panic!("overran code buffer"),
                Some(&k) => decode(k)
            };

            match opcode
            {
                 1 => self.binop(|x, y| x + y, &modes),
                 2 => self.binop(|x, y| x * y, &modes),
                 3 => self.input(&modes),
                 4 => return Some(self.output(&modes)),
                 5 => self.jump(|x| x != 0, &modes),
                 6 => self.jump(|x| x == 0, &modes),
                 7 => self.binop(|x, y| if x <  y { 1 } else { 0 }, &modes),
                 8 => self.binop(|x, y| if x == y { 1 } else { 0 }, &modes),
                 9 => self.adjust_bp(&modes),
                99 => return None,
                _  => panic!("invalid opcode: {}", opcode)
            }
        }
    }
}

fn decode(i : i64) -> (i64, Vec<u8>)
{
    let opcode = i % 100;
    let mut k  = i / 100;

    let args = match opcode { 1 => 3, 2 => 3, 3 => 1, 4 => 1, 5 => 2, 6 => 2, 7 => 3, 8 => 3, 9 => 1, _ => 0 };
    let modes  = (0 .. args).map(|_| if k == 0 { 0 } else { let m = k % 10 ; k /= 10; m as u8 }).collect();

    (opcode, modes)
}

impl<I : Iterator<Item = i64>> Interpreter<I>
{
    pub fn new(memory : Vec<i64>, input_iter : I) -> Interpreter<I>
    {
        Interpreter
        {
            memory,
            ip: 0,
            bp: 0,
            input_iter,
            send_req: None
        }
    }

    pub fn iter(&mut self) -> &mut Interpreter<I>
    {
        self
    }

    fn binop(&mut self, op : impl Fn(i64, i64) -> i64, modes : &[u8])
    {
        *self.index_modal(modes[2], self.ip + 3) = op(*self.index_modal(modes[0], self.ip + 1),
                                                      *self.index_modal(modes[1], self.ip + 2));
        self.ip += 4;
    }

    fn input(&mut self, modes : &[u8])
    {
        if let Some(req) = &self.send_req
        {
            let _ = req.send(Request::Input);
        }

        match self.input_iter.next()
        {
            Some(x) => *self.index_modal(modes[0], self.ip + 1) = x,
            None    => panic!("input iterator yielded nothing")
        }
        self.ip += 2;
    }

    fn output(&mut self, modes : &[u8]) -> i64
    {
        if let Some(req) = &self.send_req
        {
            let _ = req.send(Request::Output);
        }

        self.ip += 2;
        *self.index_modal(modes[0], self.ip - 1)
    }

    fn jump(&mut self, f : impl Fn(i64) -> bool, modes : &[u8])
    {
        if f(*self.index_modal(modes[0], self.ip + 1))
        {
            self.ip = *self.index_modal(modes[1], self.ip + 2) as usize
        }
        else
        {
            self.ip += 3
        }
    }

    fn adjust_bp(&mut self, modes : &[u8])
    {
        self.bp = (self.bp as i64 + *self.index_modal(modes[0], self.ip + 1)) as usize;
        self.ip += 2;
    }

    fn index_modal(&mut self, mode : u8, v : usize) -> &mut i64
    {
        let ix = match mode
        {
            0 => self.memory[v] as usize,
            1 => v,
            2 => (self.memory[v] + self.bp as i64) as usize,
            _ => panic!("invalid addressing mode: {}", mode)
        };

        if ix >= self.memory.len()
        {
            self.memory.extend(std::iter::repeat(0).take(1 + ix - self.memory.len()))
        }

        &mut self.memory[ix]
    }
}

impl Interpreter<Empty<i64>>
{
    pub fn with_channel(memory : Vec<i64>, recv_in : Receiver<i64>, send_out : Sender<i64>, send_req : Option<Sender<Request>>) -> JoinHandle<Interpreter<Empty<i64>>>
    {
        spawn(move ||
        {
            let mut interpreter = Interpreter::new(memory, recv_in.iter());
            interpreter.send_req = send_req;

            for o in interpreter.iter()
            {
                match send_out.send(o)
                {
                    Ok(_)  => continue,
                    Err(_) => panic!("output channel closed unexpectedly")
                }
            }

            Interpreter
            {
                memory:     interpreter.memory,
                ip:         interpreter.ip,
                bp:         interpreter.bp,
                input_iter: empty(),
                send_req:   None
            }
        })
    }

    pub fn stdio(memory : Vec<i64>) -> Interpreter<Empty<i64>>
    {
        let (send_in,  recv_in ) = channel();
        let (send_out, recv_out) = channel();
        let (send_req, recv_req) = channel();

        let handle = Interpreter::with_channel(memory, recv_in, send_out, Some(send_req));
        for r in recv_req.iter()
        {
            match r
            {
                Request::Output => if let Ok(x) = recv_out.recv() { println!("{}", x) },
                Request::Input  =>
                {
                    loop
                    {
                        print!("input: ");
                        stdout().flush().expect("failed to flush stdout");

                        let mut input = String::new();
                        stdin().read_line(&mut input).unwrap();
                        match input.trim_end().parse()
                        {
                            Ok(x)  => { send_in.send(x).unwrap(); break },
                            Err(e) => println!("parse error: {}", e)
                        }
                    }
                }
            }
        }
        handle.join().unwrap()
    }
}
