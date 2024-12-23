use std::sync::Arc;

use super::{AlignedBlock, AlignedBlockFromDefault, AlignedBlockFromIterator, BlockFetch, BlockStore, IndexedBlock};

impl<B> IndexedBlock for Arc<B>
where B: IndexedBlock
{
    type Index = B::Index;
    type Item = B::Item;
}

impl<B> AlignedBlock for Arc<B>
where B: AlignedBlock
{
    fn alignment() -> Self::Index {
        B::alignment()
    }

    fn position(&self) -> Self::Index {
        self.as_ref().position()
    }
}

impl<B> AlignedBlockFromDefault for Arc<B>
where B: AlignedBlockFromDefault
{
    fn default_per_index(position: Self::Index, value: impl Fn(Self::Index) -> Self::Item) -> Self {
        Arc::new(B::default_per_index(position, value))
    }
}

impl<B> BlockFetch for Arc<B>
where B: BlockFetch
{
    fn fetch(&self, index: Self::Index) -> Self::Item {
        self.as_ref().fetch(index)
    }
}

impl<B> BlockStore for Arc<B>
where B: BlockStore,
B: Clone
{
    fn store(&mut self, index: Self::Index, item: Self::Item) {
        Arc::make_mut(self).store(index,item);
    }
}

impl<B> AlignedBlockFromIterator for Arc<B>
where B: AlignedBlockFromIterator
{
    fn from_iterator<I>(position: Self::Index, iter: &mut I) -> Self
        where I: Iterator<Item=Self::Item> {
        Arc::new(B::from_iterator(position, iter))
    }
}