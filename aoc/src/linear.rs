pub fn bounds_1d<'a, T : 'a + Ord + Copy>(mut i : impl Iterator<Item = &'a T>) -> Option<(T, T)>
{
    i.next().map(|&m| i.fold((m, m), |(min, max), &k| (min.min(k), max.max(k))))
}

pub fn bounds_2d<'a, T : 'a + Ord + Copy>(mut i : impl Iterator<Item = &'a (T, T)>) -> Option<(T, T, T, T)>
{
    i.next().map(|&(mx, my)| i.fold((mx, my, mx, my), |(min_x, min_y, max_x, max_y), &(x, y)|
    {
        (min_x.min(x), min_y.min(y), max_x.max(x), max_y.max(y))
    }))
}
