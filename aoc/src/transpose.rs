pub fn transpose<T : Clone>(matrix : &[Vec<T>]) -> Vec<Vec<T>>
{
    let num_cols = matrix.get(0).map(Vec::len).unwrap_or(0);
    let mut rows = matrix.iter().map(|row| row.iter().cloned()).collect::<Vec<_>>();

    (0 .. num_cols).map(|_| rows.iter_mut()
                                .filter_map(|row| row.next())
                                .collect::<Vec<T>>())
                   .collect::<Vec<Vec<T>>>()
}
