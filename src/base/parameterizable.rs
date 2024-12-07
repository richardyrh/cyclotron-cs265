pub trait Parameterizable<T> {
    type Error;
    fn get_prefix() -> String;
    fn configure(&mut self, config: T) -> Result<(), Self::Error>;
}
