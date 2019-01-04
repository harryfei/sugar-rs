pub trait SResultExt<E> {
    fn drop_value(self) -> Result<(), E>;
    fn catch_err<F>(self, op: F) -> ()
    where
        F: FnOnce(E) -> ();
    fn ignore(self) -> ();
}

impl<T, E> SResultExt<E> for Result<T, E> {
    fn drop_value(self) -> Result<(), E> {
        self.map(|_| ())
    }

    fn catch_err<O>(self, op: O) -> ()
    where
        O: FnOnce(E) -> (),
    {
        let _ = self.map_err(|e| {
            op(e);
        });
    }

    fn ignore(self) -> () {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_drop_value() {
        let x: Result<u32, &str> = Err("nothing!");
        let y = x.drop_value();
        assert_eq!(y, Err("nothing!") as Result<(), &str>);
    }

    #[test]
    fn test_catch_err() {
        let x: Result<u32, &str> = Err("nothing!");
        let y = x.catch_err(|_| {});
        assert_eq!(y, ());
    }

    #[test]
    fn test_ignore() {
        let x: Result<u32, &str> = Err("nothing!");
        let y = x.ignore();
        assert_eq!(y, ());
    }

}
