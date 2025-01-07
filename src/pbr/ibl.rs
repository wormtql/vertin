use cgmath::{InnerSpace, Vector3};
use image::{ImageBuffer, Rgb, Rgba, RgbaImage};
use rand::{Rng, thread_rng};
use crate::math::{GGXDistribution, hammersley, reflect};


pub struct SchlickFresnelBaking {
    pub size: usize,
    pub sample_count: usize,
}

pub type Rgba64Image = ImageBuffer<Rgba<u16>, Vec<u16>>;


impl SchlickFresnelBaking {
    pub fn new(size: usize, sample_count: usize) -> Self {
        Self {
            size,
            sample_count
        }
    }

    pub fn bake_one_sample_use_normal_distribution(&self, cos_theta: f64, roughness: f64) -> (f64, f64) {
        assert!(cos_theta >= 0.0 && cos_theta <= 1.0);
        assert!(roughness >= 0.0 && roughness <= 1.0);

        let mut sum = 0.0;
        let mut sum2 = 0.0;
        let ggx = GGXDistribution::new(roughness);

        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let wo = Vector3::new(sin_theta, 0.0, cos_theta);
        let g1 = ggx.g1(wo);

        for i in 0..self.sample_count {
            // let r1 = thread_rng().gen_range(0.0..1.0);
            // let r2 = thread_rng().gen_range(0.0..1.0);

            let r = hammersley(i, self.sample_count);
            let r1 = r.x;
            let r2 = r.y;

            let (sampled_normal, pdf) = ggx.importance_sample_normal(r1, r2);

            let wi = reflect(wo, sampled_normal);
            if wi.z <= 0.0 {
                continue;
            }

            let g2 = ggx.smith_g2(wi, wo);
            let g_vis = g2 * wo.dot(sampled_normal) / (sampled_normal.z.max(0.0) * wo.z.max(0.0));

            let fc = (1.0 - wo.dot(sampled_normal)).powf(5.0);

            // sum += (1.0 - fc) * g_vis;
            sum += g_vis;
            sum2 += fc * g_vis;
        }

        let denom = self.sample_count as f64;
        (sum / denom, sum2 / denom)
    }

    pub fn bake_one_sample(&self, cos_theta: f64, roughness: f64) -> (f64, f64) {
        assert!(cos_theta >= 0.0 && cos_theta <= 1.0);
        assert!(roughness >= 0.0 && roughness <= 1.0);

        let mut sum = 0.0;
        let mut sum2 = 0.0;
        let ggx = GGXDistribution::new(roughness);

        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let wo = Vector3::new(sin_theta, 0.0, cos_theta);
        let g1 = ggx.g1(wo);

        for i in 0..self.sample_count {
            // let r1 = thread_rng().gen_range(0.0..1.0);
            // let r2 = thread_rng().gen_range(0.0..1.0);

            let r = hammersley(i, self.sample_count);
            let r1 = r.x;
            let r2 = r.y;

            let (sampled_normal, pdf) = ggx.importance_sample_visible_normal(wo, r1, r2);

            let mut wi = reflect(wo, sampled_normal);
            if wi.z < 0.0 {
                // wi.z = 0.0;
                // wi = wi.normalize();
                // println!("wi: {:?}, wo: {:?}, normal: {:?}, pdf: {}, roughness: {}", wi, wo, sampled_normal, pdf, roughness);
                continue;
            }
            // assert!(wi.z >= 0.0 && wi.z <= 1.0);

            let g2 = ggx.smith_g2(wi, wo);
            let g1 = ggx.g1(wi);
            let dot = wo.dot(sampled_normal);

            let f = (1.0 - dot).powf(5.0);
            let f2 = 1.0 - (1.0 - dot).powf(5.0);

            sum += g2 * f2;
            sum2 += g2 * f;
        }

        let denom = self.sample_count as f64 * g1;
        // let denom = self.sample_count as f64;
        (sum / denom, sum2 / denom)
    }

    fn f64_to_u8(v: f64) -> u8 {
        // assert!(v >= 0.0 && v <= 1.0);

        let v = v.min(1.0).max(0.0);
        (v * 255.0).round() as u8
    }

    fn f64_to_u16(v: f64) -> u16 {
        let v = v.min(1.0).max(0.0);
        (v * 65535.0).round() as u16
    }

    pub fn bake(&self) -> Rgba64Image {
        let mut result = Rgba64Image::new(self.size as u32, self.size as u32);

        for i in 0..self.size as u32 {
            for j in 0..self.size as u32 {
                let cos_theta = (i as f64 + 0.5) / (self.size as f64);
                let roughness = (j as f64 + 0.5) / (self.size as f64);

                let value = self.bake_one_sample(cos_theta, roughness * roughness);
                // let value = self.bake_one_sample_use_normal_distribution(cos_theta, roughness * roughness);
                // println!("{}, {}: {}", i, j, value);
                let r = Self::f64_to_u16(value.0);
                let g = Self::f64_to_u16(value.1);

                if i as usize == self.size - 1 && j as usize == self.size - 1 {
                    println!("{:?}", value);
                }

                result.put_pixel(i, self.size as u32 - j - 1, Rgba([r, g, 0, 65535]));
            }
        }

        result
    }
}
