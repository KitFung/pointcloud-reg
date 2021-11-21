use std::rc::Rc;

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
type Point3D = Point<FieldType>;

struct TreeNode<const Dim: usize> {
    // data: Rc<Vec<Point3D>>
}

type PointPtr<const Dim: usize> = Box<TreeNode<Dim>>;

#[derive(Default, Clone, Copy)]
struct Bounding {
    low: FieldType,
    high: FieldType,
}

type BoundingBox<const Dim: usize> = [Bounding; Dim];

pub struct KdTree<const Dim: usize> {
    dataset: Vec<Point3D>,
    /// Create KdTree Object and Build Kdtree Index is 2 step
    root_node: Option<PointPtr<Dim>>,
    root_bbox: BoundingBox<Dim>,
}

impl<const Dim: usize> KdTree<Dim> {
    pub fn new(dataset: Vec<Point3D>) -> KdTree<Dim> {
        assert!(!dataset.is_empty(), "Not Support empty pointset");
        Self {
            dataset,
            root_node: None,
            root_bbox: [Bounding::default(); Dim],
        }
    }

    pub fn build_index(&mut self) -> &Self {
        if self.root_node.is_some() {
            // TODO or just error?
            self.root_node = None;
        }

        self.compute_bounding_box();

        let mut bbox = self.root_bbox;
        self.root_node = Some(self.divide_tree(0, self.dataset.len(), &mut bbox));
        self.root_bbox = bbox;

        self
    }

    fn compute_bounding_box(&mut self) {
        let pts = self.dataset.as_slice();

        let field = pts[0].as_slice();
        for i in 0..Dim {
            self.root_bbox[i].low = field[i];
            self.root_bbox[i].high = field[i];
        }

        for pt in pts {
            let field = pt.as_slice();
            for i in 0..Dim {
                self.root_bbox[i].low = self.root_bbox[i].low.min(field[i]);
                self.root_bbox[i].high = self.root_bbox[i].high.max(field[i]);
            }
        }
    }

    fn divide_tree(
        &mut self,
        left_offset: usize,
        right_offset: usize,
        bbox: &mut BoundingBox<Dim>,
    ) -> Box<TreeNode<Dim>> {
        Box::new(TreeNode::<Dim> {})
    }

    fn split_at_middle(&mut self, ind: usize) {}

    fn split_plane(&mut self, ind: usize, count: usize, curfeat: i32) {}

    fn compute_min_max(&mut self, ind: usize, count: usize, dim: usize) -> (FieldType, FieldType) {
        (0.0, 0.0)
    }
}

impl<const Dim: usize> KdTree<Dim> {
    pub fn query_with_bounding_box(&self, bbox: &[FieldType; Dim]) -> Vec<Point3D> {
        Vec::new()
    }
    pub fn query_k_nearest_neighor(&self, target: &Point3D, k_neighbor: u32) -> Vec<Point3D> {
        Vec::new()
    }
}
