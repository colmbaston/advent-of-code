fn main()
{
    let input      = include_str!("../input.txt").trim_end().parse::<usize>().unwrap();
    let mut houses = vec![0 ; input / 10];

    for i in 1 ..= input / 10
    {
        for j in (i ..= input / 10).step_by(i)
        {
            houses[j-1] += i * 10;
        }
    }
    println!("{}", houses.iter().zip(1 ..).find(|x| *x.0 >= input).unwrap().1);

    for h in houses.iter_mut() { *h = 0 }
    for i in 1 ..= input / 10
    {
        for j in (i ..= input / 10).step_by(i).take(50)
        {
            houses[j-1] += i * 11;
        }
    }
    println!("{}", houses.iter().zip(1 ..).find(|x| *x.0 >= input).unwrap().1);
}
