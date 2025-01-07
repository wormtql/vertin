use std::f64::consts::PI;
use cgmath::{InnerSpace, Vector3};
use crate::math::common_sample::sample_uniform_disk_polar;
use crate::math::reflect;
use crate::math::utils::lerp;

pub struct GGXDistribution {
    pub roughness: f64,
}

impl GGXDistribution {
    pub fn new(roughness: f64) -> Self {
        Self {
            roughness
        }
    }

    /// Evaluate the normal distribution function
    /// The input normal must be in the positive hemisphere (w.z > 0)
    ///
    /// - `w`: The normal
    pub fn normal_distribution_function(&self, w: Vector3<f64>) -> f64 {
        let a2 = self.roughness * self.roughness;
        let cos = w.z;

        let d = 1.0 + cos * cos * (a2 - 1.0);
        a2 / (PI * d * d)
    }

    /// A helper function to calculate the shadowing/masking term
    /// $ \Lambda(s) $
    pub fn lambda(&self, dir: Vector3<f64>) -> f64 {
        let cos_theta = dir.z;
        let cos_theta_2 = cos_theta * cos_theta;
        if cos_theta_2 == 0.0 {
            return 0.0;
        }
        let sin_theta_2 = 1.0 - cos_theta_2;
        let tan_theta_2 = sin_theta_2 / cos_theta_2;
        let a2 = self.roughness * self.roughness;

        ((1.0 + a2 * tan_theta_2).sqrt() - 1.0) * 0.5
    }

    /// The shadowing/masking function
    pub fn g1(&self, w: Vector3<f64>) -> f64 {
        1.0 / (1.0 + self.lambda(w))
    }

    /// Smith height-correlated masking-shadowing function
    pub fn smith_g2(&self, wi: Vector3<f64>, wo: Vector3<f64>) -> f64 {
        1.0 / (1.0 + self.lambda(wi) + self.lambda(wo))
    }

    pub fn visible_normal_distribution_function(&self, wo: Vector3<f64>, n: Vector3<f64>) -> f64 {
        let cos_theta_o = wo.z;
        assert!(cos_theta_o >= 0.0 && cos_theta_o <= 1.0);

        let g1 = self.g1(wo);
        let normal_distribution = self.normal_distribution_function(n);
        g1 / cos_theta_o * normal_distribution * (wo.dot(n).abs())
    }

    /// Importance sample a visible normal, given a incidence/view direction
    /// The function assumes the geometry normal is (0, 0, 1)
    /// The given direction have to be in the hemisphere where w.z > 0
    ///
    /// * `w` - Given direction
    /// * `r1` and `r2` - 2 uniform random numbers
    ///
    /// Returns the sampled normal and PDF
    // pub fn importance_sample_visible_normal(&self, w: Vector3<f64>, r1: f64, r2: f64) -> (Vector3<f64>, f64) {
    //     assert!(w.z >= 0.0);
    //
    //     // Special case for roughness == 0
    //     if self.roughness == 0.0 {
    //         return (Vector3::new(0.0, 0.0, 1.0), 1.0)
    //     }
    //
    //     let mut wh = Vector3::new(self.roughness * w.x, self.roughness * w.y, w.z).normalize();
    //
    //     // tangent1
    //     let t1 = if wh.z < 0.9999999 {
    //         let z = Vector3::new(0.0, 0.0, 1.0);
    //         z.cross(wh).normalize()
    //     } else {
    //         Vector3::new(1.0, 0.0, 0.0)
    //     };
    //     // tangent2
    //     let t2 = wh.cross(t1);
    //
    //     let mut p = sample_uniform_disk_polar(r1, r2);
    //     let h = (1.0 - p.x * p.x).sqrt();
    //     p.y = lerp(h, p.y, (1.0 + wh.z) / 2.0);
    //
    //     let length_2 = p.x * p.x + p.y * p.y;
    //     let pz = (1.0 - length_2).max(0.0).sqrt();
    //     let nh = t1 * p.x + t2 * p.y + wh * pz;
    //
    //     let n = Vector3::new(self.roughness * nh.x, self.roughness * nh.y, nh.z.max(1e-6)).normalize();
    //     assert!(n.z >= 0.0);
    //     // if n.dot(w) < 0.0 {
    //     //     println!("n: {:?}, w: {:?}, dot: {}, roughness: {}", n, w, n.dot(w), self.roughness);
    //     // }
    //     // assert!(n.dot(w) >= 0.0);
    //
    //     let pdf = self.visible_normal_distribution_function(w, n);
    //
    //     (n, pdf)
    // }

    pub fn importance_sample_visible_normal(&self, w: Vector3<f64>, r1: f64, r2: f64) -> (Vector3<f64>, f64) {
        assert!(w.z >= 0.0);

        // Special case for roughness == 0
        if self.roughness == 0.0 {
            return (Vector3::new(0.0, 0.0, 1.0), 1.0)
        }

        let mut wh = Vector3::new(self.roughness * w.x, self.roughness * w.y, w.z).normalize();
        let lensq = wh.x * wh.x + wh.y * wh.y;

        let t1 = if lensq > 0.0 { Vector3::new(-wh.y, wh.x, 0.0) * (1.0 / lensq.sqrt()) } else { Vector3::new(1.0, 0.0, 0.0) };
        let t2 = wh.cross(t1);

        let mut p = sample_uniform_disk_polar(r1.sqrt(), r2);
        let h = (1.0 - p.x * p.x).sqrt();
        p.y = lerp(h, p.y, (1.0 + wh.z) / 2.0);

        let length_2 = p.x * p.x + p.y * p.y;
        let pz = (1.0 - length_2).max(0.0).sqrt();
        let nh = t1 * p.x + t2 * p.y + wh * pz;

        let n = Vector3::new(self.roughness * nh.x, self.roughness * nh.y, nh.z.max(0.0)).normalize();
        assert!(n.z >= 0.0);
        // if n.dot(w) < 0.0 {
        //     println!("n: {:?}, w: {:?}, dot: {}, roughness: {}", n, w, n.dot(w), self.roughness);
        // }
        // assert!(n.dot(w) >= 0.0);

        let pdf = self.visible_normal_distribution_function(w, n);

        (n, pdf)
    }

    /// Importance sample a normal according to the GGX normal distribution function
    /// The geometry normal is assumed to be (0, 0, 1)
    /// And the sampled normal will be in the positive hemisphere
    ///
    /// Returns the sampled direction and PDF
    pub fn importance_sample_normal(&self, r1: f64, r2: f64) -> (Vector3<f64>, f64) {
        assert!(r1 <= 1.0 && r1 >= 0.0);
        assert!(r2 <= 1.0 && r2 >= 0.0);

        let phi = 2.0 * PI * r1;

        let a2 = self.roughness * self.roughness;
        assert!(a2 >= 0.0 && a2 <= 1.0);

        let v = (1.0 - r2) / (r2 * (a2 - 1.0) + 1.0);
        let cos_theta = v.sqrt();
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let result = Vector3::new(
            sin_theta * phi.cos(),
            sin_theta * phi.sin(),
            cos_theta
        );

        let d = (cos_theta * a2 - cos_theta) * cos_theta + 1.0;
        let normal_distribution = a2 / (PI * d * d);
        let pdf = normal_distribution * cos_theta;

        (result, pdf)
    }
}
