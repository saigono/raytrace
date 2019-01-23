pub use self::hitable::{HitRecord, Hitable};
pub use self::hitable_list::HitableList;
pub use self::material::{Material, Lambertian, Metal};
pub use self::sphere::Sphere;

pub mod hitable;
pub mod hitable_list;
pub mod material;
pub mod sphere;
