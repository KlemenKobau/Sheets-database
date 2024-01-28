pub trait Entity {
    fn save(&self);

    fn query(match_function: dyn Fn(&Self) -> bool) -> Vec<Self>
    where
        Self: std::marker::Sized;
}
