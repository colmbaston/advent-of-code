pub fn transpose<'a, T : Clone + 'a>(mut matrix : impl Iterator<Item = &'a [T]>) -> impl Iterator<Item = Vec<T>> + 'a
{
    let row      = matrix.next().unwrap_or(&[]);
    let num_cols = row.len();
    let mut rows = std::iter::once(row).chain(matrix)
                                       .map(|row| row.iter().cloned())
                                       .collect::<Vec<_>>();

    (0 .. num_cols).map(move |_| rows.iter_mut()
                                     .filter_map(|row| row.next())
                                     .collect::<Vec<T>>())
}
