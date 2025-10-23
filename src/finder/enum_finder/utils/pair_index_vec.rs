use std::{
    ops::{Index, IndexMut},
    vec::IntoIter,
};

pub trait PairIndex: Copy {
    fn pair_index(&self) -> (usize, usize);
    fn from_pair_index(pair: (usize, usize)) -> Self;
}

#[derive(Clone, Default, Debug)]
pub struct PairIndexVec<I, T> {
    indices: Vec<I>,
    raw: Vec<Vec<Option<T>>>,
}

impl<I, T> PairIndexVec<I, T>
where
    I: PairIndex,
    T: Clone,
{
    pub fn new() -> Self {
        Self {
            indices: Vec::new(),
            raw: Vec::new(),
        }
    }

    pub fn set(&mut self, index: I, item: T) {
        let (outer, inner) = index.pair_index();
        while self.raw.len() <= outer {
            self.raw.push(Vec::new());
        }
        while self.raw[outer].len() <= inner {
            self.raw[outer].push(None);
        }
        if self.raw[outer][inner].is_none() {
            self.indices.push(index);
        }
        self.raw[outer][inner] = Some(item);
    }

    pub fn get(&self, index: I) -> Option<&T> {
        let (outer, inner) = index.pair_index();
        self.raw.get(outer)?.get(inner)?.as_ref()
    }

    pub fn get_mut(&mut self, index: I) -> Option<&mut T> {
        let (outer, inner) = index.pair_index();
        self.raw.get_mut(outer)?.get_mut(inner)?.as_mut()
    }

    pub fn iter(&self) -> impl Iterator<Item = (I, &T)> {
        self.indices
            .iter()
            .filter_map(|index| self.get(*index).map(|item| (*index, item)))
    }
}

impl<I, T> FromIterator<(I, T)> for PairIndexVec<I, T>
where
    I: PairIndex,
    T: Clone,
{
    fn from_iter<U: IntoIterator<Item = (I, T)>>(iter: U) -> Self {
        let mut pair_index_vec = PairIndexVec::new();
        for (index, item) in iter {
            pair_index_vec.set(index, item);
        }
        pair_index_vec
    }
}

impl<I, T> IntoIterator for PairIndexVec<I, T>
where
    I: PairIndex,
    T: Clone,
{
    type IntoIter = std::vec::IntoIter<(I, T)>;
    type Item = (I, T);

    fn into_iter(self) -> Self::IntoIter {
        self.indices
            .iter()
            .filter_map(|index| self.get(*index).map(|item| (*index, item.clone())))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl<I, T> Index<I> for PairIndexVec<I, T>
where
    I: PairIndex,
    T: Clone,
{
    type Output = T;

    fn index(&self, index: I) -> &T {
        self.get(index).expect("Index out of bounds")
    }
}

impl<I, T> IndexMut<I> for PairIndexVec<I, T>
where
    I: PairIndex,
    T: Clone,
{
    fn index_mut(&mut self, index: I) -> &mut T {
        self.get_mut(index).expect("Index out of bounds")
    }
}

#[cfg(test)]
mod test {
    use crate::finder::enum_finder::utils::pair_index_vec::{PairIndex, PairIndexVec};

    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    struct Index(usize, usize);

    impl PairIndex for Index {
        fn pair_index(&self) -> (usize, usize) {
            (self.0, self.1)
        }

        fn from_pair_index(pair: (usize, usize)) -> Self {
            Self(pair.0, pair.1)
        }
    }

    #[test]
    fn test_pair_index_vec() {
        let mut vec = PairIndexVec::new();
        vec.set(Index(0, 0), 1);
        vec.set(Index(0, 1), 2);
        vec.set(Index(1, 0), 3);

        assert_eq!(vec.get(Index(0, 0)), Some(&1));
        assert_eq!(vec.get(Index(0, 1)), Some(&2));
        assert_eq!(vec.get(Index(1, 0)), Some(&3));
        assert_eq!(vec.get(Index(1, 1)), None);

        let vec2 = vec.clone();
        let mut it = vec2.iter();
        assert_eq!(it.next(), Some((Index(0, 0), &1)));
        assert_eq!(it.next(), Some((Index(0, 1), &2)));
        assert_eq!(it.next(), Some((Index(1, 0), &3)));
        assert_eq!(it.next(), None);

        assert_eq!(vec[Index(0, 0)], 1);
        assert_eq!(vec[Index(0, 1)], 2);
        assert_eq!(vec[Index(1, 0)], 3);

        vec[Index(0, 0)] = 10;
        assert_eq!(vec.get(Index(0, 0)), Some(&10));
        assert_eq!(vec[Index(0, 0)], 10);
    }
}
