use super::{
    utils::random_i32,
    vec3::{Point3, Precision, Vec3},
};

const POINT_COUNT: usize = 256;

#[derive(Clone, Debug)]
pub struct PerlinNoise {
    randvec: [Vec3; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl PerlinNoise {
    pub fn new() -> Self {
        let mut randvec = [Vec3::default(); POINT_COUNT];
        for val in randvec.iter_mut() {
            *val = Vec3::random_bounded(-1., 1.).unit_vec();
        }

        let mut perm_x = [0; POINT_COUNT]; 
        let mut perm_y = [0; POINT_COUNT];
        let mut perm_z = [0; POINT_COUNT];

        generate_perm(&mut perm_x);
        generate_perm(&mut perm_y);
        generate_perm(&mut perm_z);

        Self {
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Point3) -> Precision {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for di in 0..2i32 {
            for dj in 0..2i32 {
                for dk in 0..2i32 {
                    let ii = (i+di) & (POINT_COUNT-1) as i32;
                    let jj = (j+dj) & (POINT_COUNT-1) as i32;
                    let kk = (k+dk) & (POINT_COUNT-1) as i32;
                    c[di as usize][dj as usize][dk as usize] = self.randvec[(
                        self.perm_x[ii as usize] ^
                        self.perm_y[jj as usize] ^
                        self.perm_z[kk as usize]
                    ) as usize]
                }
            }
        }

        perlin_interp(c, u, v, w)
    }

    pub fn turbulence(&self, p: Point3, depth: i32) -> Precision {
        let mut accum = 0.;
        let mut temp_p = p;
        let mut weight = 1.;

        for _i in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.;
        }

        accum.abs()
    }
}

fn generate_perm(perm: &mut [i32; POINT_COUNT]) {
    for (i, val) in perm.iter_mut().enumerate() {
        *val = i as i32;
    }
    permute(perm);
}

fn permute(p: &mut [i32; POINT_COUNT]) {
    for i in (0..POINT_COUNT).rev() {
        let target = random_i32(0, i as i32);
        p.swap(i, target as usize);
    }
}

fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: Precision, v: Precision, w: Precision) -> Precision {
    let uu = u*u*(3. - 2.*u);
    let vv = v*v*(3. - 2.*v);
    let ww = w*w*(3. - 2.*w);
    let mut accum = 0.;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v = Vec3::new(u - i as Precision, v - j as Precision, w - k as Precision);
                accum += (i as Precision *uu + (1-i) as Precision * (1.-uu))
                    * (j as Precision *vv + (1-j) as Precision * (1.-vv))
                    * (k as Precision *ww + (1-k) as Precision * (1.-ww))
                    * c[i][j][k].dot(&weight_v);
            }
        }
    }

    accum
}

impl Default for PerlinNoise {
    fn default() -> Self {
        PerlinNoise::new()
    }
}
