use crate::bounding::{BoundingNumber, BoxBounded};
use crate::quadtree::data::QuadTreeData;
use crate::rect::Rect;

use smallvec::SmallVec;
use stable_vec::StableVec;

pub struct QuadTreeNode<T> {
    pub bb: Rect<T>,
    pub children: Vec<QuadTreeNode<T>>,
    pub data: SmallVec<[usize; 8]>,
}

impl<T> QuadTreeNode<T>
where
    T: BoundingNumber,
{
    pub(crate) fn new(bb: Rect<T>) -> Self {
        QuadTreeNode {
            bb: bb,
            children: Vec::with_capacity(4),
            data: SmallVec::new(),
        }
    }

    pub fn visit<DataT, F>(&self, data: &StableVec<DataT>, mut f: F)
    where
        F: FnMut(QuadTreeData<T, DataT>),
    {
        f(QuadTreeData::new(&self, data));

        for child in &self.children {
            child.visit(data, &mut f);
        }
    }
}

impl<T> BoxBounded<T> for QuadTreeNode<T>
where
    T: BoundingNumber,
{
    fn get_bounding_box(&self) -> Rect<T> {
        return self.bb;
    }
}

impl<T> QuadTreeNode<T>
where
    T: BoundingNumber,
{
    fn split(&mut self) {
        let two: T = T::from_u8(2).unwrap();
        let bb = &self.bb;
        let half_w = bb.w / two;
        let half_h = bb.h / two;
        let top_left = Rect {
            x: bb.x,
            y: bb.y,
            w: half_w,
            h: half_h,
        };
        let top_right = Rect {
            x: bb.x + half_w,
            y: bb.y,
            w: half_w,
            h: half_h,
        };
        let bottom_right = Rect {
            x: bb.x + half_w,
            y: bb.y + half_h,
            w: half_w,
            h: half_h,
        };
        let bottom_left = Rect {
            x: bb.x,
            y: bb.y + half_h,
            w: half_w,
            h: half_h,
        };
        self.children = vec![
            QuadTreeNode::new(top_left),
            QuadTreeNode::new(top_right),
            QuadTreeNode::new(bottom_right),
            QuadTreeNode::new(bottom_left),
        ];
    }

    fn try_children<DataT, const SPLIT_LIMIT: usize>(
        &mut self,
        bb: &Rect<T>,
        storage: &StableVec<DataT>,
        index: usize,
    ) -> Option<usize>
    where
        DataT: BoxBounded<T>,
    {
        let mut maybe_index = Option::from(index);
        for child in &mut self.children {
            match maybe_index {
                None => break,
                Some(x) => maybe_index = child.try_insert::<DataT, SPLIT_LIMIT>(bb, storage, x),
            }
        }
        return maybe_index;
    }

    pub(crate) fn try_insert<DataT, const SPLIT_LIMIT: usize>(
        &mut self,
        bb: &Rect<T>,
        storage: &StableVec<DataT>,
        index: usize,
    ) -> Option<usize>
    where
        DataT: BoxBounded<T>,
    {
        if !bb.fits_inside(&self.bb) {
            return Some(index);
        }

        if !self.children.is_empty() {
            if let Some(unused) = self.try_children::<DataT, SPLIT_LIMIT>(bb, storage, index) {
                self.data.push(unused);
            }
            return None;
        }

        if self.data.len() > SPLIT_LIMIT {
            self.split();
            let mut remaining = Vec::new();
            while let Some(id) = self.data.pop() {
                unsafe {
                    let data = storage.get_unchecked(id);
                    let old_bb = data.get_bounding_box();
                    if let Some(unused) =
                        self.try_children::<DataT, SPLIT_LIMIT>(&old_bb, storage, id)
                    {
                        remaining.push(unused);
                    }
                }
            }

            for id in remaining {
                self.data.push(id);
            }
            if let Some(x) = self.try_children::<DataT, SPLIT_LIMIT>(bb, storage, index) {
                self.data.push(x);
            }
            return None;
        }

        self.data.push(index);
        return None;
    }
}
