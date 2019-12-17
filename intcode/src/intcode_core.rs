use std::sync::mpsc::{ SyncSender, Receiver };

pub type Memory = Vec<i64>;
pub enum Request { Input, Output }

pub fn interpret(memory : &mut Memory, recv_in : Receiver<i64>, send_out : SyncSender<i64>, req : Option<SyncSender<Request>>)
{
    let mut ip = 0;
    let mut bp = 0;
    loop
    {
        let (opcode, modes) = match memory.get(ip)
        {
            None    => panic!("overran code buffer"),
            Some(k) => decode(k)
        };

        match opcode
        {
            01 => binop(|x, y| x + y,                      &modes, &mut ip, &bp, memory),
            02 => binop(|x, y| x * y,                      &modes, &mut ip, &bp, memory),
            03 => input(                                   &modes, &mut ip, &bp, memory, &recv_in,  &req),
            04 => output(                                  &modes, &mut ip, &bp, memory, &send_out, &req),
            05 => jump(|x| x != 0,                         &modes, &mut ip, &bp, memory),
            06 => jump(|x| x == 0,                         &modes, &mut ip, &bp, memory),
            07 => binop(|x, y| if x <  y { 1 } else { 0 }, &modes, &mut ip, &bp, memory),
            08 => binop(|x, y| if x == y { 1 } else { 0 }, &modes, &mut ip, &bp, memory),
            09 => { bp = (bp as i64 + *index_modal(modes[0], ip+1, &bp, memory)) as usize ; ip += 2 },
            99 => break,
            _  => panic!("invalid opcode: {}", opcode)
        }
    }
}

type Modes  = Vec<u8>;

fn decode(i : &i64) -> (i64, Modes)
{
    let opcode = i % 100;
    let mut k  = i / 100;

    let args = match opcode
    {
        01 => 3,
        02 => 3,
        03 => 1,
        04 => 1,
        05 => 2,
        06 => 2,
        07 => 3,
        08 => 3,
        09 => 1,
        _  => 0
    };
    let modes  = (0 .. args).map(|_| if k == 0 { 0 } else { let m = k % 10 ; k /= 10; m as u8 }).collect();

    (opcode, modes)
}

fn binop<F : Fn(i64, i64) -> i64>(op : F, modes : &Modes, ip : &mut usize, bp : &usize, memory : &mut Memory)
{
    *index_modal(modes[2], *ip+3, bp, memory) = op(*index_modal(modes[0], *ip+1, bp, memory),
                                                   *index_modal(modes[1], *ip+2, bp, memory));
    *ip += 4
}

fn input(modes : &Modes, ip : &mut usize, bp : &usize, memory : &mut Memory, recv_in : &Receiver<i64>, req : &Option<SyncSender<Request>>)
{
    if let Some(req) = req
    {
        let _ = req.send(Request::Input);
    }

    match recv_in.recv()
    {
        Ok(x)  => *index_modal(modes[0], *ip+1, bp, memory) = x,
        Err(_) => panic!("input channel closed unexpectedly")
    }
    *ip    += 2;
}

fn output(modes : &Modes, ip : &mut usize, bp : &usize, memory : &mut Memory, send_out : &SyncSender<i64>, req : &Option<SyncSender<Request>>)
{
    if let Some(req) = req
    {
        let _ = req.send(Request::Output);
    }

    if let Err(_) = send_out.send(*index_modal(modes[0], *ip+1, bp, memory))
    {
        panic!("output channel closed unexpectedly")
    }
    *ip += 2
}

fn jump<F : Fn(i64) -> bool>(f : F, modes : &Modes, ip : &mut usize, bp : &usize, memory : &mut Memory)
{
    if f(*index_modal(modes[0], *ip+1, bp, memory))
    {
        *ip = *index_modal(modes[1], *ip+2, bp, memory) as usize
    }
    else
    {
        *ip += 3
    }
}

fn index_modal<'m>(mode : u8, v : usize, bp : &usize, memory : &'m mut Memory) -> &'m mut i64
{
    let ix = match mode
    {
        0 => memory[v] as usize,
        1 => v,
        2 => (memory[v] + *bp as i64) as usize,
        _ => panic!("invalid addressing mode: {}", mode)
    };

    if ix >= memory.len()
    {
        memory.extend(std::iter::repeat(0).take(1 + ix - memory.len()))
    }

    &mut memory[ix]
}
