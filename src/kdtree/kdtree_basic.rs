use std::{rc::Rc, usize};

#[repr(C)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point<T>
where
    T: Copy,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        assert!(std::mem::size_of::<Self>() <= isize::MAX as _);
        unsafe { std::slice::from_raw_parts(self as *const _ as *const T, 3) }
    }
}

type FieldType = f32;
type DistanceType = f32;
type Point3D = Point<FieldType>;

/// The Actual Data is Stored in KdTree.
/// This only keep the index of the corresponding data
/// Datas corresponding to one data will be located sequentially in the actual data array
struct LeafNodeData {
    /// Start offset of the datas
    left_offset: usize,
    /// End offset of the datas
    right_offset: usize,
}

struct NonLeafNodeData {
    /// Dimension used for div
    divdim: usize,
    /// Value used for sub div
    divlow: DistanceType,
    /// Value used for sub div
    divhigh: DistanceType,
}

enum TreeNodeData {
    // data: Rc<Vec<Point3D>>
    LeafNodeData(LeafNodeData),
    NonLeafNodeData(NonLeafNodeData),
    None,
}

struct TreeNode {
    /// The internal data
    data: TreeNodeData,
    /// Left Child Node
    left_child: Option<Box<TreeNode>>,
    /// Right Child Node
    right_child: Option<Box<TreeNode>>,
}

impl Default for TreeNode {
    fn default() -> Self {
        Self {
            data: TreeNodeData::None,
            left_child: None,
            right_child: None,
        }
    }
}

impl TreeNode {
    pub fn new() -> Self {
        TreeNode::default()
    }

    pub fn new_leafnode(left_offset: usize, right_offset: usize) -> Self {
        Self {
            data: TreeNodeData::LeafNodeData(LeafNodeData {
                left_offset,
                right_offset,
            }),
            ..Default::default()
        }
    }

    pub fn new_non_leafnode(
        left_child: Box<TreeNode>,
        right_child: Box<TreeNode>,
        divdim: usize,
        divlow: DistanceType,
        divhigh: DistanceType,
    ) -> Self {
        Self {
            left_child,
            right_child,
            data: TreeNodeData::LeafNodeData(NonLeafNodeData {
                divdim,
                divlow,
                divhigh,
            }),
            ..Default::default()
        }
    }
}

type PointPtr = Box<TreeNode>;

#[derive(Default, Clone, Copy)]
struct Bounding {
    low: FieldType,
    high: FieldType,
}

type BoundingBox<const DIM: usize> = [Bounding; DIM];

pub struct KdTree<const DIM: usize> {
    dataset: Vec<Point3D>,
    /// Create KdTree Object and Build Kdtree Index is 2 step
    root_node: Option<PointPtr>,
    root_bbox: BoundingBox<DIM>,
    /// If least or equal to m_leaf_max_size points, store as leafnode
    m_leaf_max_size: usize,
}

impl<const DIM: usize> KdTree<DIM> {
    pub fn new(dataset: Vec<Point3D>) -> KdTree<DIM> {
        assert!(!dataset.is_empty(), "Not Support empty pointset");
        Self {
            dataset,
            root_node: None,
            root_bbox: [Bounding::default(); DIM],
            m_leaf_max_size: 10,
        }
    }

    pub fn build_index(&mut self) -> &Self {
        // Ensure old status is invalided
        self.root_node = None;
        self.compute_bounding_box();

        let mut bbox = self.root_bbox;
        self.root_node = Some(self.divide_tree(0, self.dataset.len(), &mut bbox));
        self.root_bbox = bbox;

        self
    }

    fn compute_bounding_box_for_range(
        &self,
        left: usize,
        right: usize,
        bbox: &mut BoundingBox<DIM>,
    ) {
        let pts = self.dataset[left + 1..right].iter();

        let field = self.dataset[left].as_slice();
        for i in 0..DIM {
            bbox[i].low = field[i];
            bbox[i].high = field[i];
        }

        for pt in pts {
            let field = pt.as_slice();
            for i in 0..DIM {
                bbox[i].low = bbox[i].low.min(field[i]);
                bbox[i].high = bbox[i].high.max(field[i]);
            }
        }
    }

    fn compute_bounding_box(&mut self) {
        let mut bbox = self.root_bbox;
        self.compute_bounding_box_for_range(0, self.dataset.len(), &mut bbox);
        self.root_bbox = bbox;
    }

    /// The bbox is used for middleSplit Compute
    /// Pass by reference is because we want to avoid unnecessary creation
    /// Handle Points index: left_offset <= x < right_offset
    fn divide_tree(
        &mut self,
        left_offset: usize,
        right_offset: usize,
        bbox: &mut BoundingBox<DIM>,
    ) -> Box<TreeNode> {
        if left_offset + self.m_leaf_max_size > right_offset {
            // Compute BoundingBox
            self.compute_bounding_box_for_range(left_offset, right_offset + 1, bbox);

            // Store as leafnode
            Box::new(TreeNode::new_leafnode(left_offset, right_offset))
        } else {
            // Split the plane
            let (ind, cutdim, cutval) =
                self.split_at_middle(left_offset, right_offset - left_offset, &bbox);

            let mut left_bbox = bbox.clone();
            let left_node = self.divide_tree(left_offset, left_offset + ind, &mut left_bbox);

            let mut right_bbox = bbox.clone();
            let right_node = self.divide_tree(left_offset + ind, right_offset, &mut right_bbox);

            Box::new(TreeNode::new_non_leafnode(
                left_node,
                right_node,
                cutdim,
                left_bbox[cutdim].low,
                right_bbox[cutdim].high,
            ))
        }
    }

    /// Return (split index, cut DIMension, cut value)
    fn split_at_middle(
        &mut self,
        ind: usize,
        count: usize,
        &bbox: &BoundingBox<DIM>,
    ) -> (usize, usize, DistanceType) {
        let eps = 0.00001 as DistanceType;
        let iter = self.dataset.iter().skip(ind).take(count);
        let dim_iter = (0..DIM).map(|i| bbox[i].high - bbox[i].low);
        let max_span = dim_iter.fold(0.0 as DistanceType, |l, r| l.max(r));
        panic!("")
    }

    fn split_plane(&mut self, ind: usize, count: usize, curfeat: i32) {}

    fn compute_min_max(&mut self, ind: usize, count: usize, dim: usize) -> (FieldType, FieldType) {
        (0.0, 0.0)
    }
}

impl<const DIM: usize> KdTree<DIM> {
    pub fn query_with_bounding_box(&self, bbox: &[FieldType; DIM]) -> Vec<Point3D> {
        Vec::new()
    }
    pub fn query_k_nearest_neighor(&self, target: &Point3D, k_neighbor: u32) -> Vec<Point3D> {
        Vec::new()
    }
}
