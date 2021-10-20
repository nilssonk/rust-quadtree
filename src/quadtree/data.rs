use crate::quadtree::node::QuadTreeNode;
use smallvec::SmallVec;
use stable_vec::StableVec;

pub struct QuadTreeData<'a, T, DataT> {
    pub node: &'a QuadTreeNode<T>,
    pub data: SmallVec<[&'a DataT; 8]>,
}

impl<'a, T, DataT> QuadTreeData<'a, T, DataT> {
    pub(crate) fn new(
        node: &'a QuadTreeNode<T>,
        data: &'a StableVec<DataT>,
    ) -> QuadTreeData<'a, T, DataT> {
        let refs = node
            .data
            .iter()
            .map(|index| unsafe {
                return data.get_unchecked(*index);
            })
            .collect();

        return QuadTreeData {
            node: node,
            data: refs,
        };
    }
}
