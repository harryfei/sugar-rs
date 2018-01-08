pub trait SResultExt<E> {
    fn eat_value(self) -> Result<(), E>;
}

impl<T, E> SResultExt<E> for Result<T, E> {
    fn eat_value(self) -> Result<(), E> {
        self.map(|_| {
            ()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        let x: Result<u32, &str> = Err("nothing!");
        let y = x.eat_value();
        assert_eq!(y, Err("nothing!") as Result<(), &str>);
    }
}