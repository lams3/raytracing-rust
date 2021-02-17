use crate::structures::{Vec3, Point3};

use rand::prelude::{thread_rng, Rng};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    random_vec: [Vec3; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT]
}

impl Perlin {
    pub fn new() -> Self {
        let mut random_vec = [Vec3::zero(); POINT_COUNT];

        for i in 0..POINT_COUNT {
            random_vec[i] = Vec3::random_in_unit_sphere();
        }

        Self {
            random_vec: random_vec,
            perm_x: Perlin::generate_perm(),
            perm_y: Perlin::generate_perm(),
            perm_z: Perlin::generate_perm()
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        unsafe {
            let u = p.x - f64::floor(p.x);
            let v = p.y - f64::floor(p.y);
            let w = p.z - f64::floor(p.z);

            let i = f64::to_int_unchecked::<usize>(f64::floor(p.x));
            let j = f64::to_int_unchecked::<usize>(f64::floor(p.y));
            let k = f64::to_int_unchecked::<usize>(f64::floor(p.z));

            let mut arr = [[[Vec3::zero(); 2]; 2]; 2];

            for di in 0..2 {
                for dj in 0..2 {
                    for dk in 0..2 {
                        let ii = usize::wrapping_add(i, di) & 255;
                        let jj = usize::wrapping_add(j, dj) & 255;
                        let kk = usize::wrapping_add(k, dk) & 255;
                        let index = (self.perm_x[ii] ^ self.perm_y[jj] ^ self.perm_z[kk]) as usize;
                        arr[di][dj][dk] = self.random_vec[index];
                    }
                }
            }

            Perlin::trilinear_interp(arr, u, v, w)
        }
    }

    pub fn turbulence(&self, p: Point3) -> f64{
        self.turbulence_with_depth(p, 7)
    }

    pub fn turbulence_with_depth(&self, p: Point3, depth: u32) -> f64 {
        let mut accum = 0.0;
        let mut weight = 1.0;
        let mut p = p;

        for _ in 0..depth {
            accum += weight * self.noise(p);
            weight *= 0.5;
            p = 2.0 * p;
        }

        f64::abs(accum)
    }

    fn generate_perm() -> [i32; POINT_COUNT] {
        let mut arr = [0; POINT_COUNT];

        for i in 0..POINT_COUNT {
            arr[i] = i as i32;
        }

        Perlin::permute(&mut arr);

        arr
    }

    fn permute(arr: &mut [i32; POINT_COUNT]) {
        let mut rng = thread_rng();

        for i in (1..POINT_COUNT).rev() {
            let j = rng.gen_range(0..i);
            let temp = arr[i];
            arr[i] = arr[j];
            arr[j] = temp;
        }
    }

    fn trilinear_interp(arr: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        
        let mut acc = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let (ii, jj, kk) = (i as f64, j as f64, k as f64);
                    let weight_vec = Vec3::new(u - ii, v - jj, w - kk);
                    acc += 
                        ((ii * uu) + (1.0 - ii) * (1.0 - uu)) *
                        ((jj * vv) + (1.0 - jj) * (1.0 - vv)) *
                        ((kk * ww) + (1.0 - kk) * (1.0 - ww)) *
                        Vec3::dot(&arr[i][j][k], &weight_vec);
                }
            }
        }

        acc
    }
}