
pub trait Resolver<R> {

    fn resolve(&mut self) -> (f32, Vec<bool>, f32);

    fn multiple_resolve(&mut self, number_execution: i32) -> Vec<R>;

}

