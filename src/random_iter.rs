use rand::thread_rng;
use rand::seq::SliceRandom;
use smallvec::SmallVec;
pub struct RandomIter<T>{
    items: SmallVec<[T; 20]>
}

impl<T> RandomIter<T>{
    pub fn from_iter(iter: impl Iterator<Item=T>) -> RandomIter<T>{
        let mut items: SmallVec<[T;20]> = iter.collect();
        items.shuffle(&mut thread_rng());
        RandomIter{
            items
        }
    }
}

impl From<std::ops::Range<usize>> for RandomIter<usize>{
    fn from(range: std::ops::Range<usize>) -> Self {
        RandomIter::from_iter(range.into_iter())
    }
}

impl<'a,T> From<&'a[T]> for RandomIter<&'a T>{
    fn from(array: &'a [T]) -> Self {
        RandomIter::from_iter(array.into_iter())
    }
}

impl<T> Iterator for RandomIter<T>{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.items.pop()
    }
}