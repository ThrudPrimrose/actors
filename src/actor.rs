pub trait Acts {
    fn act(&self);
}

//The user provided
pub struct Actor<T: Acts> {
    pub core: T,
}

impl<T: Acts> Actor<T> {
    pub fn act(&self) {
        self.core.act();
    }
}