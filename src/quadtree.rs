mod data;
mod node;

pub use crate::bounding::{BoundingNumber, BoxBounded};
pub use crate::rect::Rect;
pub use data::QuadTreeData;
pub use node::QuadTreeNode;

use stable_vec::StableVec;

/// # Summary
/// A generic quadtree for type DataT.
///
/// # Type parameters
/// * `T` - Type parameter of the queried Axis-Aligned Bounding Boxes (Rect\<T>), must implement BoundingNumber\<T>.
/// * `DataT` - The data type stored in the tree. Must implement BoxBounded\<T>.
/// * `const SPLIT_LIMIT: usize` - The maximum number of elements per node before subdivision occurs.
///
/// # Internals
/// Data is stored in an ostensibly cache-friendly manner in a central StableVec with the nodes containing indexes into it.
pub struct QuadTree<T, DataT, const SPLIT_LIMIT: usize> {
    storage: StableVec<DataT>,
    root: QuadTreeNode<T>,
}

impl<T, DataT, const SPLIT_LIMIT: usize> QuadTree<T, DataT, SPLIT_LIMIT>
where
    T: BoundingNumber,
    DataT: BoxBounded<T>,
{
    /// # Summary
    /// Constructs a new QuadTree.
    ///
    /// # Parameters
    /// * `bounding_box` - An Axis-Aligned Bounding Box (Rect\<T>) that describes the extent of the QuadTree.
    ///
    /// # Examples
    /// ```ignore
    /// let bounding_box: Rect<i32> = Rect {
    ///     x: 0,
    ///     y: 0,
    ///     w: 512,
    ///     h: 512,
    /// };
    /// let qt : QuadTree<i32, Foo> = QuadTree::new(bounding_box);
    /// ```
    pub fn new(bounding_box: Rect<T>) -> Self {
        QuadTree {
            storage: StableVec::new(),
            root: QuadTreeNode::new(bounding_box),
        }
    }

    /// # Summary
    /// Applies a visitor function recursively to each node in the QuadTree.
    ///
    /// Note: For each internal node, the children are visited in geometrically clockwise order starting with top-left (lowest x and y).
    ///
    /// # Parameters
    /// * `f` - Visitor function to apply recursively. Must implement FnMut(&QuadTreeNode<T>, &StableVec<DataT>).
    pub fn visit<F>(&self, f: F)
    where
        F: FnMut(QuadTreeData<T, DataT>),
    {
        self.root.visit(&self.storage, f);
    }

    /// # Summary
    /// Attempts to insert an element of type `DataT`, returning an `Ok(&mut DataT)` if successful,
    /// or an `Err(DataT)` if the element's bounding box does not fit inside the `QuadTree`'s top level bounding box.
    ///
    /// # Parameters
    /// * `data` - An element of type `DataT`
    ///
    /// # Examples
    /// ```ignore
    /// let foo: Bar::new(); // Bar implements BoxBounded<i32>
    ///
    /// let bounding_box: Rect<i32> = Rect {
    ///     x: 0,
    ///     y: 0,
    ///     w: 512,
    ///     h: 512,
    /// };
    /// let qt = QuadTree::new(bounding_box);
    /// match qt.try_insert(foo) {
    ///     Ok(inserted) => *inserted.bar += 1, // Mutable reference to the inserted element
    ///     Err(unused) => println!("Unable to insert element {:?}", unused), // Moved original element
    /// }
    /// ```
    pub fn try_insert(&mut self, data: DataT) -> Result<&mut DataT, DataT> {
        let index = self.storage.push(data);
        unsafe {
            let bb = self.storage.get_unchecked(index).get_bounding_box();
            if let Some(unused_index) =
                self.root
                    .try_insert::<DataT, SPLIT_LIMIT>(&bb, &self.storage, index)
            {
                return Err(self.storage.remove(unused_index).unwrap());
            }

            return Ok(&mut self.storage[index]);
        }
    }
}
