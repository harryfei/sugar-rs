use std::hash::{BuildHasher, Hash};
use std::collections::HashMap;
use std::borrow::Borrow;

pub trait SHashMapExt<K, V> {
    fn get_clone<Q: ?Sized>(&self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
        V: Clone;
}

impl<K, V, S> SHashMapExt<K, V> for HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    fn get_clone<Q: ?Sized>(&self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
        V: Clone,
    {
        self.get(k).map(|v| (*v).clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // In plain Rust, the code below can't build
        // let _ = {
        //     let mut map = HashMap::new();
        //     map.insert(1, "1");
        //     map.get(&1)
        // };

        // After using the SHashmapExt, we can build these code below.
        let _ = {
            let mut map = HashMap::new();
            map.insert(1, "1");
            map.get_clone(&1)
        };
    }
}
