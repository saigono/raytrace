use super::aabb::AABB;
use super::hittable::{HitRecord, Hittable};
use crate::linalg::{Ray, Vec3};
use crate::materials::Material;

use std::sync::Arc;

pub struct Triangle {
    v_index: [usize; 3],
    n_index: [usize; 3],
    vertices: Arc<Vec<Vec3>>,
    normals: Arc<Vec<Vec3>>,
    material: Arc<dyn Material>,
}

impl Triangle {
    pub fn new(
        v0: usize,
        v1: usize,
        v2: usize,
        n0: usize,
        n1: usize,
        n2: usize,
        vertices: Arc<Vec<Vec3>>,
        normals: Arc<Vec<Vec3>>,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            v_index: [v0, v1, v2],
            n_index: [n0, n1, n2],
            vertices: vertices,
            normals: normals,
            material: material,
        }
    }
}

#[inline(always)]
fn fmax(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

#[inline(always)]
fn fmin(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let e1 = self.vertices[self.v_index[1]] - self.vertices[self.v_index[0]];
        let e2 = self.vertices[self.v_index[2]] - self.vertices[self.v_index[0]];
        let q = Vec3::cross(&r.direction, &e2);
        let a = Vec3::dot(&e1, &q);
        if a > -0.00001 && a < 0.00001 {
            return None;
        }

        let f = 1.0 / a;
        let s = r.origin - self.vertices[self.v_index[0]];
        let u = f * Vec3::dot(&s, &q);
        if u < 0.0 {
            return None;
        }
        let o = Vec3::cross(&s, &e1);
        let v = f * Vec3::dot(&r.direction, &o);
        if v < 0.0 || (u + v) > 1.0 {
            return None;
        }
        let t = f * Vec3::dot(&e2, &o);

        if t < t_min || t > t_max {
            return None;
        }

        let w = 1.0 - u - v;
        let normal = u * self.normals[self.n_index[1]]
            + v * self.normals[self.n_index[2]]
            + w * self.normals[self.n_index[0]];

        let p = r.point_at_parameter(t);
        if Vec3::dot(&normal, &r.direction) >= 0.0 {
            None
        } else {
            Some(HitRecord::new(t, u, v, p, normal, self.material.clone()))
        }
    }
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let min = Vec3::new(
            fmin(
                self.vertices[self.v_index[0]][0],
                fmin(
                    self.vertices[self.v_index[1]][0],
                    self.vertices[self.v_index[2]][0],
                ),
            ),
            fmin(
                self.vertices[self.v_index[0]][1],
                fmin(
                    self.vertices[self.v_index[1]][1],
                    self.vertices[self.v_index[2]][1],
                ),
            ),
            fmin(
                self.vertices[self.v_index[0]][2],
                fmin(
                    self.vertices[self.v_index[1]][2],
                    self.vertices[self.v_index[2]][2],
                ),
            ),
        );

        let max = Vec3::new(
            fmax(
                self.vertices[self.v_index[0]][0],
                fmax(
                    self.vertices[self.v_index[1]][0],
                    self.vertices[self.v_index[2]][0],
                ),
            ),
            fmax(
                self.vertices[self.v_index[0]][1],
                fmax(
                    self.vertices[self.v_index[1]][1],
                    self.vertices[self.v_index[2]][1],
                ),
            ),
            fmax(
                self.vertices[self.v_index[0]][2],
                fmax(
                    self.vertices[self.v_index[1]][2],
                    self.vertices[self.v_index[2]][2],
                ),
            ),
        );
        Some(AABB::new(&min, &max))
    }
}

pub struct TriangleMesh {
    n_triangles: usize,
    vertices: Arc<Vec<Vec3>>,
    normals: Arc<Vec<Vec3>>,
    v_index: Vec<usize>,
    n_index: Vec<usize>,
    material: Arc<dyn Material>,
}

impl TriangleMesh {
    pub fn new(
        n_triangles: usize,
        vertices: Arc<Vec<Vec3>>,
        normals: Arc<Vec<Vec3>>,
        v_index: Vec<usize>,
        n_index: Vec<usize>,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            n_triangles: n_triangles,
            vertices: vertices,
            normals: normals,
            v_index: v_index,
            n_index: n_index,
            material: material,
        }
    }

    pub fn iter<'a>(&'a self) -> TriangleMeshIterator<'a> {
        TriangleMeshIterator {
            cnt: 0,
            inner: self,
        }
    }
}

pub struct TriangleMeshIterator<'a> {
    cnt: usize,
    inner: &'a TriangleMesh,
}

impl<'a> Iterator for TriangleMeshIterator<'a> {
    type Item = Triangle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cnt >= self.inner.n_triangles {
            None
        } else {
            let _normal = (self.inner.normals[self.inner.n_index[self.cnt * 3] - 1]
                + self.inner.normals[self.inner.n_index[self.cnt * 3 + 1] - 1]
                + self.inner.normals[self.inner.n_index[self.cnt * 3 + 2] - 1])
                / 3.0;
            // let triangle = Triangle::new(
            //     &self.inner.vertices[self.inner.v_index[self.cnt * 3] - 1],
            //     &self.inner.vertices[self.inner.v_index[self.cnt * 3 + 1] - 1],
            //     &self.inner.vertices[self.inner.v_index[self.cnt * 3 + 2] - 1],
            //     &normal,
            //     self.inner.material.clone(),
            // );
            let triangle = Triangle::new(
                self.inner.v_index[self.cnt * 3] - 1,
                self.inner.v_index[self.cnt * 3 + 1] - 1,
                self.inner.v_index[self.cnt * 3 + 2] - 1,
                self.inner.n_index[self.cnt * 3] - 1,
                self.inner.n_index[self.cnt * 3 + 1] - 1,
                self.inner.n_index[self.cnt * 3 + 2] - 1,
                self.inner.vertices.clone(),
                self.inner.normals.clone(),
                self.inner.material.clone(),
            );
            self.cnt += 1;
            Some(triangle)
        }
    }
}

// {
//             "type": "triangle",
//             "vertices": [
//                 [
//                     5.0,
//                     0.0,
//                     1.0
//                 ],
//                 [
//                     5.0,
//                     0.0,
//                     2.0
//                 ],
//                 [
//                     5.5,
//                     1.0,
//                     1.0
//                 ]
//             ],
//             "material": "red_metalic"
//         },
