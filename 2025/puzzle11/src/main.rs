use std::collections::HashMap;

fn main()
{
    let graph = parse_graph(include_str!("../input.txt"));

    let mut cache = HashMap::new();
    println!("{}", dfs("you", "out", &graph, &mut cache));

    cache.clear();
    let svr_dac = dfs("svr", "dac", &graph, &mut cache);
    cache.clear();
    let dac_fft = dfs("dac", "fft", &graph, &mut cache);
    cache.clear();
    let fft_out = dfs("fft", "out", &graph, &mut cache);
    cache.clear();
    let svr_fft = dfs("svr", "fft", &graph, &mut cache);
    cache.clear();
    let fft_dac = dfs("fft", "dac", &graph, &mut cache);
    cache.clear();
    let dac_out = dfs("dac", "out", &graph, &mut cache);
    println!("{}", svr_dac * dac_fft * fft_out
                 + svr_fft * fft_dac * dac_out);
}

fn parse_graph(s : &str) -> HashMap<&str, Vec<&str>>
{
    let mut graph = HashMap::new();
    for l in s.lines()
    {
        let (a, b) = l.split_once(": ").unwrap();
        graph.insert(a, b.split_whitespace().collect());
    }
    graph
}

fn dfs<'a>(current : &'a str, target : &str, graph : &HashMap<&str, Vec<&'a str>>, cache : &mut HashMap<&'a str, u64>) -> u64
{
    if let Some(&k) = cache.get(current) { return k }
    if current == target { return 1 }

    let mut sum = 0;
    if let Some(a) = graph.get(current)
    {
        for v in a.iter()
        {
            sum += dfs(v, target, graph, cache)
        }
    }
    cache.insert(current, sum);
    sum
}
