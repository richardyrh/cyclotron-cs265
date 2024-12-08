pub trait Parameterizable<T: TryFrom<String>> {
    type Error;
    fn get_prefixes() -> Vec<String>;
    fn configure(&mut self, prefix: &str, config: T) -> Result<(), Self::Error>;
}
