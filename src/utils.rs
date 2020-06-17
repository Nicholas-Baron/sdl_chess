pub fn map_tuple<T, R, F: Fn(T) -> R>((lhs, rhs): (T, T), func: F) -> (R, R) {
    (func(lhs), func(rhs))
}
