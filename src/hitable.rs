use crate::vec3::{Vec3};
use crate::ray::{Ray};

#[derive(Clone, Copy)]
 pub struct HitRecord
 {
     pub t: f32,
     pub p: Vec3,
     pub normal: Vec3
 }

impl HitRecord
{
    pub fn new(t: f32, p: Vec3, normal: Vec3) -> HitRecord
    {
        HitRecord{t, p, normal}
    }
}

 pub trait Hitable
 {
     fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
 }

 pub struct HitableList
 {
     pub list: Vec<Box<dyn Hitable>>
 }

 impl HitableList
{
    pub fn new(list: Vec<Box<dyn Hitable>>) -> HitableList { HitableList { list } }
    pub fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool
    {
        let mut temp_rec: HitRecord = HitRecord::new(0.0,
                                                     Vec3::new(0.0,0.0,0.0),
                                                     Vec3::new(0.0,0.0,0.0));
        let mut hit_anything: bool = false;
        let mut closest_so_far: f32 = t_max;
        for list_iter in self.list.iter()
        {
            if list_iter.hit(r, t_min, closest_so_far, &mut temp_rec)
            {
                hit_anything = true;
                closest_so_far = temp_rec.t.clone();
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
 }