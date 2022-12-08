pub fn transpose<'a, T : Clone + 'a>(matrix : impl Iterator<Item = &'a [T]>) -> impl Iterator<Item = Vec<T>> + 'a
{
    let mut matrix = matrix.fuse();
    let row_zero   = matrix.next();
    let num_cols   = row_zero.map(|s| s.len()).unwrap_or(0);
    let mut rows   = row_zero.into_iter().chain(matrix)
                             .map(|row| row.iter().cloned())
                             .collect::<Vec<_>>();

    (0 .. num_cols).map(move |_| rows.iter_mut()
                                     .filter_map(|row| row.next())
                                     .collect::<Vec<T>>())
}
