use cgmath::{Euler, Matrix4, Quaternion, Rad, Vector3};

pub trait ModelMatrix {
    /// Returns the model matrix for the transform.
    fn to_matrix(&self) -> Matrix4<f32>;
}

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub scale: Vector3<f32>,
    pub pivot: Vector3<f32>,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            pivot: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}

impl Transform {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the position of the transform.
    pub fn with_position(mut self, x: f32, y: f32, z: f32) -> Self {
        self.position = Vector3::new(x, y, z);
        self
    }

    /// Sets the position of the transform using a Vector3.
    pub fn with_position_vector(mut self, position: Vector3<f32>) -> Self {
        self.position = position;
        self
    }

    /// Sets the rotation of the transform using Euler angles in degrees at the specified anchor point.
    pub fn with_rotation(mut self, pitch: f32, yaw: f32, roll: f32) -> Self {
        self.rotation = Quaternion::from(Euler {
            x: Rad(pitch.to_radians()),
            y: Rad(yaw.to_radians()),
            z: Rad(roll.to_radians()),
        });
        self
    }

    /// Sets the scale of the transform.
    pub fn with_scale(mut self, x: f32, y: f32, z: f32) -> Self {
        self.scale = Vector3::new(x, y, z);
        self
    }

    /// Sets the pivot point of the transform.
    pub fn with_pivot(mut self, pivot: Vector3<f32>) -> Self {
        self.pivot = pivot;
        self
    }
}

impl ModelMatrix for Transform {

    /// Calculates and returns the model matrix that combines translation, rotation, and scale.
    /// The matrix is constructed in the order of translation to origin, rotation, scaling, and then translation back to the pivot point.
    fn to_matrix(&self) -> Matrix4<f32> {
        let translate_to_origin = Matrix4::from_translation(-self.pivot);
        let rotate = Matrix4::from(self.rotation);
        let scale = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
        let translate_back = Matrix4::from_translation(self.pivot);
        let translate_position = Matrix4::from_translation(self.position);
        translate_position * translate_back * rotate * scale * translate_to_origin
    }
}