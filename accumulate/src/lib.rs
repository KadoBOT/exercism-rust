pub fn map<T, U, V: FnMut(T) -> U>(input: Vec<T>, function: V) -> Vec<U> {
    input.into_iter().map(function).collect()
}
