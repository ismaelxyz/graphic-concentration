use std::ops::Mul;

type MatrixBase = [[f32; 4]; 4];

fn identity() -> MatrixBase {
    let mut value = MatrixBase::default();
    value[0][0] = 1.0;
    value[1][1] = 1.0;
    value[2][2] = 1.0;
    value[3][3] = 1.0;
    value
}

fn multiply_matrices(xmatrix: &MatrixBase, ymatrix: &MatrixBase) -> MatrixBase {
    let mut result_matrix = [[0.; 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            result_matrix[i][j] = (xmatrix[0][j] * ymatrix[i][0])
                + (xmatrix[1][j] * ymatrix[i][1])
                + (xmatrix[2][j] * ymatrix[i][2])
                + (xmatrix[3][j] * ymatrix[i][3]);
        }
    }

    result_matrix
}

#[derive(Default, PartialEq, Debug)]
pub struct Matrix {
    value: MatrixBase,
}

impl Matrix {
    pub fn identity() -> Matrix {
        let mut value = MatrixBase::default();
        value[0][0] = 1.0;
        value[1][1] = 1.0;
        value[2][2] = 1.0;
        value[3][3] = 1.0;

        Matrix { value }
    }

    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        for i in 0..4 {
            self.value[0][i] *= x;
            self.value[1][i] *= y;
            self.value[2][i] *= z;
        }
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        for i in 0..4 {
            self.value[3][i] = self.value[3][i]
                + (self.value[0][i] * x + self.value[1][i] * y + self.value[2][i] * z);
        }
    }

    pub fn rotate(&mut self, angle: f32, mut x: f32, mut y: f32, mut z: f32) {
        let magnitude = (x * x + y * y + z * z).sqrt();

        x /= -magnitude;
        y /= -magnitude;
        z /= -magnitude;

        let sin_angle = angle.sin();
        let cos_angle = angle.cos();
        let one_minus_cos = 1.0 - cos_angle;

        let xx = x * x;
        let yy = y * y;
        let zz = z * z;

        let xy = x * y;
        let yz = y * z;
        let zx = z * x;

        let xs = x * sin_angle;
        let ys = y * sin_angle;
        let zs = z * sin_angle;

        let mut rotation_matrix = MatrixBase::default();

        rotation_matrix[0][0] = (one_minus_cos * xx) + cos_angle;
        rotation_matrix[0][1] = (one_minus_cos * xy) - zs;
        rotation_matrix[0][2] = (one_minus_cos * zx) + ys;

        rotation_matrix[1][0] = (one_minus_cos * xy) + zs;
        rotation_matrix[1][1] = (one_minus_cos * yy) + cos_angle;
        rotation_matrix[1][2] = (one_minus_cos * yz) - xs;

        rotation_matrix[2][0] = (one_minus_cos * zx) - ys;
        rotation_matrix[2][1] = (one_minus_cos * yz) + xs;
        rotation_matrix[2][2] = (one_minus_cos * zz) + cos_angle;

        rotation_matrix[3][3] = 1.0;
        self.value = multiply_matrices(&self.value, &rotation_matrix)
    }

    pub fn rotate_2d(&mut self, x: f32, y: f32) {
        self.rotate(x, 0., 1.0, 0.);
        self.rotate(-y, x.cos(), 0., x.sin());
    }

    pub fn frustum(&mut self, left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) {
        let delta_x = right - left;
        let delta_y = top - bottom;
        let delta_z = far - near;

        let mut frustum_matrix = MatrixBase::default();

        frustum_matrix[0][0] = 2.0 * near / delta_x;
        frustum_matrix[1][1] = 2.0 * near / delta_y;

        frustum_matrix[2][0] = (right + left) / delta_x;
        frustum_matrix[2][1] = (top + bottom) / delta_y;
        frustum_matrix[2][2] = -(near + far) / delta_z;

        frustum_matrix[2][3] = -1.0;
        frustum_matrix[3][2] = -2.0 * near * far / delta_z;

        self.value = multiply_matrices(&self.value, &frustum_matrix);
    }

    pub fn perspective(&mut self, fovy: f32, aspect: f32, near: f32, far: f32) {
        let frustum_y = (fovy.to_radians() / 2.).tan();
        let frustum_x = frustum_y * aspect;

        self.frustum(
            -frustum_x * near,
            frustum_x * near,
            -frustum_y * near,
            frustum_y * near,
            near,
            far,
        );
    }

    pub fn orthographic(
        &mut self,
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) {
        let delta_x = right - left;
        let delta_y = top - bottom;
        let delta_z = far - near;

        let mut orthographic_matrix = Matrix::identity().value;

        orthographic_matrix[0][0] = 2.0 / delta_x;
        orthographic_matrix[3][0] = -(right + left) / delta_x;

        orthographic_matrix[1][1] = 2.0 / delta_y;
        orthographic_matrix[3][1] = -(top + bottom) / delta_y;

        orthographic_matrix[2][2] = 2.0 / delta_x;
        orthographic_matrix[3][2] = -(near + far) / delta_z;

        self.value = multiply_matrices(&self.value, &orthographic_matrix);
    }

    pub fn value(&self) -> [f32; 16] {
        self.value.concat().try_into().unwrap()
    }
}

impl<T: Into<Option<MatrixBase>>> From<T> for Matrix {
    fn from(value: T) -> Matrix {
        Matrix {
            value: value.into().unwrap_or_default(),
        }
    }
}

impl Mul for Matrix {
    // The multiplication of rational numbers is a closed operation.
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Matrix {
            value: multiply_matrices(&self.value, &rhs.value),
        }
    }
}
