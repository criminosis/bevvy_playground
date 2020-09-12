use bevy::prelude::Vec3;
pub trait Vec3Ext {
    ///Multiple the a Vec3 by the given scalar
    fn scalar_multiply(&mut self, scalar: f32);
}

impl Vec3Ext for Vec3 {
    fn scalar_multiply(&mut self, scalar: f32) {
        *self.x_mut() *= scalar;
        *self.y_mut() *= scalar;
        *self.z_mut() *= scalar;
    }
}
