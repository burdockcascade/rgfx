use cgmath::{ortho, perspective, Deg, InnerSpace, Matrix4, Point3, SquareMatrix, Vector3};

pub trait CameraMatrix {
    fn to_view_projection_matrix(&self) -> Matrix4<f32>;
}

#[derive(Debug, Clone)]
pub enum CameraMovement {
    Forward(f32),
    Backward(f32),
    Left(f32),
    Right(f32),
    Up(f32),
    Down(f32),
}

#[derive(Debug, Clone)]
pub enum Projection {
    Perspective {
        fov_y: Deg<f32>,
        near: f32,
        far: f32,
        scale: f32,
        aspect: f32,
    },
    Orthographic {
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
        scale: f32,
        aspect: f32,
    },
}

#[derive(Clone, Debug)]
pub struct Camera {
    pub position: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub projection: Projection,
    view_projection_matrix: Matrix4<f32>
}

impl Camera {

    pub fn default_perspective(width: f32, height: f32) -> Self {
        Self {
            position: Point3::new(0.0, 0.0, 5.0),
            target: Point3::new(0.0, 0.0, 0.0),
            up: Vector3::unit_y(),
            projection: Projection::Perspective {
                fov_y: Deg(45.0),
                near: 1.0,
                far: 100.0,
                aspect: width / height,
                scale: 1.0,
            },
            view_projection_matrix: Matrix4::identity()
        }
    }

    pub fn default_orthographic(width: f32, height: f32) -> Self {
        Self {
            position: Point3::new(0.0, 0.0, 100.0),
            target: Point3::new(0.0, 0.0, 0.0),
            up: Vector3::unit_y(),
            projection: Projection::Orthographic {
                left: -1.0,
                right: 1.0,
                bottom: -1.0,
                top: 1.0,
                near: 0.0,
                far: 100.0,
                aspect: width / height,
                scale: 1.0,
            },
            view_projection_matrix: Matrix4::identity()
        }
    }

    fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(self.position, self.target, self.up)
    }

    fn projection_matrix(&self) -> Matrix4<f32> {
        match self.projection {
            Projection::Perspective { fov_y, near, far, scale, aspect } => {
                perspective(fov_y, aspect, near, far)
            }
            Projection::Orthographic { left, right, bottom, top, near, far, scale, aspect } => {

                let half_h = 1.0;
                let half_w = half_h * aspect;

                ortho(
                    -half_w, half_w,
                    -half_h, half_h,
                    near, far,
                )
            }
        }
    }

    pub fn update_viewport(&mut self, width: f32, height: f32) {
        match &mut self.projection {
            Projection::Perspective { aspect, .. } => {
                *aspect = width / height;
            }
            Projection::Orthographic { aspect, .. } => {
                *aspect = width / height;
            }
        }
        self.update_view_projection_matrix();
    }

    pub fn move_camera(&mut self, movement: CameraMovement) {
        let forward = (self.target - self.position).normalize();
        let right = forward.cross(self.up).normalize();

        match movement {
            CameraMovement::Forward(amount) => {
                self.position += forward * amount;
                self.target += forward * amount;
            }
            CameraMovement::Backward(amount) => {
                self.position -= forward * amount;
                self.target -= forward * amount;
            }
            CameraMovement::Left(amount) => {
                self.position -= right * amount;
                self.target -= right * amount;
            }
            CameraMovement::Right(amount) => {
                self.position += right * amount;
                self.target += right * amount;
            }
            CameraMovement::Up(amount) => {
                self.position += self.up * amount;
                self.target += self.up * amount;
            }
            CameraMovement::Down(amount) => {
                self.position -= self.up * amount;
                self.target -= self.up * amount;
            }
        }

        self.update_view_projection_matrix();
    }

    fn update_view_projection_matrix(&mut self) {
        let view = self.view_matrix();
        let projection = self.projection_matrix();
        self.view_projection_matrix = projection * view;
    }

}

impl CameraMatrix for Camera {
    fn to_view_projection_matrix(&self) -> Matrix4<f32> {
        self.view_projection_matrix
    }
}
