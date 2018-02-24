pub trait SResultExt<E> {
    fn drop_value(self) -> Result<(), E>;
}

impl<T, E> SResultExt<E> for Result<T, E> {
    fn drop_value(self) -> Result<(), E> {
        self.map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let x: Result<u32, &str> = Err("nothing!");
        let y = x.drop_value();
        assert_eq!(y, Err("nothing!") as Result<(), &str>);
    }
}
