pub use maplit::*;
pub use vec_box::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let v = vec![1, 2, 3];
        let map = hashmap!(
            1 => "1".to_owned(),
            2 => "2".to_owned(),
            3 => "3".to_owned(),
        );

        assert_eq!(map, m!{i => i.to_string(), for i in v})
    }
}
