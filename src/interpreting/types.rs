use std::fmt::Debug;

pub trait QType: Debug {}

impl QType for f64 {}
impl QType for String {}
