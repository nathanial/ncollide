use nalgebra::na::{Dim, Cast};
use nalgebra::na;
use geom::mesh::MeshElement;

#[deriving(Encodable, Decodable, Clone)]
pub struct Segment<N, V> {
    margin: N,
    a:      V,
    b:      V
}

impl<N: Cast<f32>, V: Dim> Segment<N, V> {
    #[inline]
    pub fn new(a: V, b: V) -> Segment<N, V> {
        Segment::new_with_margin(a, b, na::cast(0.04))
    }

    #[inline]
    pub fn new_with_margin(a: V, b: V, margin: N) -> Segment<N, V> {
        assert!(na::dim::<V>() > 1);

        Segment {
            margin: margin,
            a:      a,
            b:      b
        }
    }
}

impl<N: Clone, V> Segment<N, V> {
    #[inline]
    pub fn a<'a>(&'a self) -> &'a V {
        &'a self.a
    }

    #[inline]
    pub fn b<'a>(&'a self) -> &'a V {
        &'a self.b
    }

    #[inline]
    pub fn margin(&self) -> N {
        self.margin.clone()
    }
}

impl<N: Cast<f32>, V: Dim + Clone> MeshElement<N, V> for Segment<N, V> {
    #[inline]
    fn nvertices(_: Option<Segment<N, V>>) -> uint {
        2
    }

    #[inline]
    fn new_with_vertices_and_indices(vs: &[V], is: &[uint], margin: N) -> Segment<N, V> {
        assert!(is.len() == 2);

        Segment::new_with_margin(vs[is[0]].clone(), vs[is[1]].clone(), margin)
    }
}