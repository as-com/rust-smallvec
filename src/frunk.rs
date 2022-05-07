use frunk::{labelled::{Transmogrifier, Field}, indices::MappingIndicesWrapper};

use crate::{SmallVec, Array};

impl<Key, Source, Target, InnerIndices, const M: usize, const N: usize>
    Transmogrifier<SmallVec<[Target; M]>, MappingIndicesWrapper<InnerIndices>>
    for Field<Key, SmallVec<[Source; N]>>
where
    Source: Transmogrifier<Target, InnerIndices>,
    [Target; M]: Array<Item = Target>,
    [Source; N]: Array<Item = Source>,
{
    fn transmogrify(self) -> SmallVec<[Target; M]> {
        self.value.into_iter().map(|e| e.transmogrify()).collect()
    }
}
