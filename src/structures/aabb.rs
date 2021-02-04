use crate::structures::{Point3, Ray};

#[derive(Clone, Copy, Default, Debug)]
pub struct AABB {
    pub min: Point3,
    pub max: Point3
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> Self {
        Self {
            min: min,
            max: max
        }
    }

    pub fn from_points(points: &Vec<Point3>) -> Self {
        let mut min = points[0];
        let mut max = points[0];

        for p in points {
            for i in 0..3 {
                if p[i] < min[i] {
                    min[i] = p[i]
                }

                if p[i] > max[i] {
                    max[i] = p[i]
                }
            }
        }

        Self::new(min, max)
    }

    pub fn encapsulate(&mut self, other: AABB) {
        let mut points_0 = self.get_points().iter().copied().collect();
        let mut points_1 = other.get_points().iter().copied().collect();
        let mut points = vec![];
        points.append(&mut points_0);
        points.append(&mut points_1);
        *self = AABB::from_points(&points)
    }

    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> bool {
        let mut t_min = t_min;
        let mut t_max = t_max;

        for i in 0..3 {
            let v_min = (self.min[i] - ray.origin[i]) / ray.direction[i];
            let v_max = (self.max[i] - ray.origin[i]) / ray.direction[i];

            let t0 = f64::min(v_min, v_max);
            let t1 = f64::max(v_min, v_max);

            t_min = f64::max(t0, t_min);
            t_max = f64::min(t1, t_max);

            if t_max <= t_min {
                return false
            }
        }

        true
    }

    pub fn get_points(&self) -> [Point3; 8] {
        [
            Point3::new(self.min.x, self.min.y, self.min.z),
            Point3::new(self.min.x, self.min.y, self.max.z),
            Point3::new(self.min.x, self.max.y, self.min.z),
            Point3::new(self.min.x, self.max.y, self.max.z),
            Point3::new(self.max.x, self.min.y, self.min.z),
            Point3::new(self.max.x, self.min.y, self.max.z),
            Point3::new(self.max.x, self.max.y, self.min.z),
            Point3::new(self.max.x, self.max.y, self.max.z),
        ]
    }
}