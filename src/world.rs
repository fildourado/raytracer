use crate::vec3::{Vec3};
use crate::ray::{Ray};
use crate::hitable::{HitRecord, Hitable};

pub struct Sphere
{
    pub center: Vec3,
    pub radius: f32
}

impl Sphere
{
    pub fn new(center: Vec3, radius: f32) -> Sphere
    {
        Sphere { center, radius }
    }
}

impl Hitable for Sphere
{
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool
    {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius*self.radius;
        let disc = b*b - a*c;

        if disc > 0.0
        {
            let mut temp = (-b - (b*b-a*c).sqrt())/a;
            if temp < t_max && temp > t_min
            {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
            temp = (-b + (b*b-a*c).sqrt())/a;
            if temp < t_max && temp > t_min
            {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
        }
        return false;
    }
}






