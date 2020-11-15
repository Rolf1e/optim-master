pub trait Solution<'a, Y> {
    //evaluate profit
    fn evaluate(&self, choosed_itens: &[bool]) -> Y;
}
