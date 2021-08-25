// Everything is transposed because OpenGL is weird in this regard

pub struct Vec3([f32; 3]);
#[derive(Debug)]
pub struct Mat4([f32; 16]);

impl Vec3 {
    pub fn new(xyz: [f32; 3]) -> Self {
        Vec3(xyz)
    }
}

impl Mat4 {
    pub const fn new() -> Self {
        Self([
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
        ])
    }

    pub fn mul(self, other: Self) -> Self {
        let Mat4(x) = self;
        let Mat4(y) = other;
        let mut z = [0.; 16];
        for i in 0..4 {
            for j in 0..4 {
                let mut res = 0.;
                for k in 0..4 {
                    res += x[i + k * 4] * y[k + j * 4];
                }
                z[i + j * 4] = res;
            }
        }
        Self(z)
    }

    pub fn translate(self, v: Vec3) -> Self {
        let Vec3([x, y, z]) = v;
        let trans = [1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., x, y, z, 1.0];
        self.mul(Mat4(trans))
    }

    pub fn rot_x(self, alpha: f32) -> Self {
        let sin_alpha = f32::sin(alpha);
        let cos_alpha = f32::cos(alpha);
        let rot = [
            1., 0., 0., 0., 0., cos_alpha, sin_alpha, 0., 0., -sin_alpha, cos_alpha, 0., 0., 0.,
            0., 1.,
        ];
        self.mul(Mat4(rot))
    }

    pub fn rot_y(self, alpha: f32) -> Self {
        let sin_alpha = f32::sin(alpha);
        let cos_alpha = f32::cos(alpha);
        let rot = [
            cos_alpha, 0., -sin_alpha, 0., 0., 1., 0., 0., sin_alpha, 0., cos_alpha, 0., 0., 0.,
            0., 1.,
        ];
        self.mul(Mat4(rot))
    }

    pub fn rot_z(self, alpha: f32) -> Self {
        let sin_alpha = f32::sin(alpha);
        let cos_alpha = f32::cos(alpha);
        let rot = [
            cos_alpha, sin_alpha, 0., 0., -sin_alpha, cos_alpha, 0., 0., 0., 0., 1., 0., 0., 0.,
            0., 1.,
        ];
        self.mul(Mat4(rot))
    }

    pub fn perspective(self, near: f32, far: f32, fov: f32, aspect: f32) -> Self {
        let tan = f32::tan(fov / 2.);
        let sx = 1. / (tan * aspect);
        let sy = 1. / tan;
        let sz = (far + near) / (near - far);
        let pz = (2. * far * near) / (near - far);
        self.mul(Mat4([
            sx, 0., 0., 0., 0., sy, 0., 0., 0., 0., sz, -1., 0., 0., pz, 0.,
        ]))
    }

    pub fn as_ptr(&self) -> *const f32 {
        &self.0 as *const [f32; 16] as *const _
    }
}
