pub enum BoundType {
    AxisAlignBox,
    Sphere,
}

pub trait IBound {
    fn get_type( & self ) -> BoundType;
    fn intersect( & self, other: & IBound ) -> bool;
    fn get_shortest_separation( & self, other: & IBound ) -> f64;
    fn get_bound_data( &self ) -> [f64;32];
}
