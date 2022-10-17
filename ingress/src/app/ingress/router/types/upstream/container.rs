#[derive(Debug, Clone)]
pub struct Container<T> {
    pub weight: u32,
    pub upstream: T,
}
