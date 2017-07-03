pub trait ParseCallbacks {
    type Term: From<Self::Num>;
    type Num;

    fn number(&mut self, i32) -> Self::Num;
}
