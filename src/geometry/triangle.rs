use super::aabb::{surrounding_box, AABB};
use super::hitable::{HitRecord, Hitable};
use crate::linalg::{Ray, Vec3};
use crate::materials::Material;

use std::sync::Arc;

pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    normal: Vec3,
    material: Arc<Material>,
}

impl Triangle {
    pub fn new(a: &Vec3, b: &Vec3, c: &Vec3, normal: &Vec3, material: Arc<Material>) -> Self {
        Self {
            a: *a,
            b: *b,
            c: *c,
            normal: *normal,
            material: material,
        }
    }
}

impl Hitable for Triangle {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let e1 = self.b - self.a;
        let e2 = self.c - self.a;
        let q = Vec3::cross(&r.direction, &e2);
        let a = Vec3::dot(&e1, &q);
        if a > -0.00001 && a < 0.00001 {
            return None;
        }

        let f = 1.0 / a;
        let s = r.origin - self.a;
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

        let p = r.point_at_parameter(t);
        if Vec3::dot(&self.normal, &r.direction) >= 0.0 {
            None
        } else {
            Some(HitRecord::new(
                t,
                u,
                v,
                p,
                self.normal,
                self.material.clone(),
            ))
        }
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let box1 = AABB::new(&self.a.clone(), &self.b.clone());
        let box2 = AABB::new(&self.a.clone(), &self.c.clone());
        let box3 = AABB::new(&self.b.clone(), &self.c.clone());

        let aabb = surrounding_box(&surrounding_box(&box1, &box2), &box3);
        // println!("{:?}", aabb);
        Some(aabb)
    }
}

pub struct TriangleMesh {
    n_triangles: usize,
    vertices: Vec<Vec3>,
    normals: Vec<Vec3>,
    v_index: Vec<usize>,
    n_index: Vec<usize>,
    material: Arc<Material>,
}

impl TriangleMesh {
    pub fn new(
        n_triangles: usize,
        vertices: Vec<Vec3>,
        normals: Vec<Vec3>,
        v_index: Vec<usize>,
        n_index: Vec<usize>,
        material: Arc<Material>,
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
            let normal = (self.inner.normals[self.inner.n_index[self.cnt * 3] - 1]
                + self.inner.normals[self.inner.n_index[self.cnt * 3 + 1] - 1]
                + self.inner.normals[self.inner.n_index[self.cnt * 3 + 2] - 1])
                / 3.0;
            let triangle = Triangle::new(
                &self.inner.vertices[self.inner.v_index[self.cnt * 3] - 1],
                &self.inner.vertices[self.inner.v_index[self.cnt * 3 + 1] - 1],
                &self.inner.vertices[self.inner.v_index[self.cnt * 3 + 2] - 1],
                &normal,
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
