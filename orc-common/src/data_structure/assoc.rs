use std::borrow::Borrow;

use smallvec::SmallVec;

pub trait AssocExt<K, V>
where
    K: Eq,
{
    #[inline]
    fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Eq,
    {
        self.get(k).is_some()
    }

    fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq;

    fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Eq;
}

impl<K: Eq, V> AssocExt<K, V> for Vec<(K, V)> {
    #[inline]
    fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq,
    {
        self.iter()
            .find_map(|(key, value)| (key.borrow() == k).then(|| value))
    }

    #[inline]
    fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Eq,
    {
        self.iter_mut()
            .find_map(|(key, value)| ((*key).borrow() == k).then(|| value))
    }
}

impl<K: Eq, V, const N: usize> AssocExt<K, V> for SmallVec<[(K, V); N]> {
    #[inline]
    fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq,
    {
        self.iter()
            .find_map(|(key, value)| (key.borrow() == k).then(|| value))
    }

    #[inline]
    fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Eq,
    {
        self.iter_mut()
            .find_map(|(key, value)| ((*key).borrow() == k).then(|| value))
    }
}
