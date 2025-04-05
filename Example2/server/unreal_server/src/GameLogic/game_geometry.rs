use std::fmt::Pointer;

#[derive(Clone)]
pub struct Rotation
{
    q_x : f64,
    q_y : f64,
    q_z : f64,
    q_w : f64
}

impl Rotation
{
    pub fn new_zero() -> Self {
        return Rotation { q_x: 0.0, q_y: 0.0, q_z: 0.0, q_w: 0.0 }
    }

    pub fn new(_x : f64, _y : f64, _z : f64, _w : f64) -> Self {
        return Rotation { q_x: _x, q_y: _y, q_z: _z, q_w: _w }
    }

    pub fn set_x(&mut self, _x : f64) { self.q_x = _x; }
    pub fn set_y(&mut self, _y : f64) { self.q_y = _y; }
    pub fn set_z(&mut self, _z : f64) { self.q_z = _z; }
    pub fn set_w(&mut self, _w : f64) { self.q_w = _w; }
    pub fn get_x(&self) -> f64 {return self.q_x}
    pub fn get_y(&self) -> f64 {return self.q_y}
    pub fn get_z(&self) -> f64 {return self.q_z}
    pub fn get_w(&self) -> f64 {return self.q_w}
}

#[derive(Clone)]
pub struct Position
{
    x : f64,
    y : f64,
    z : f64
}

impl Position {
    pub fn new_zero() -> Self {
        return Position { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn new(_x : f64, _y : f64, _z : f64, _w : f64) -> Self {
        return Position { x: _x, y: _y, z: _z}
    }

    pub fn set_x(&mut self, _x : f64) { self.x = _x; }
    pub fn set_y(&mut self, _y : f64) { self.y = _y; }
    pub fn set_z(&mut self, _z : f64) { self.z = _z; }
    pub fn get_x(&self) -> f64 {return self.x}
    pub fn get_y(&self) -> f64 {return self.y}
    pub fn get_z(&self) -> f64 {return self.z}
}

pub struct Transform
{
    position : Position,
    rotation : Rotation
}

impl Transform 
{
    pub fn new_zero() -> Self {
        return Transform{
            position : Position::new_zero(), 
            rotation : Rotation::new_zero() 
        }
    }

    pub fn new(_pos : Position, _rot : Rotation) -> Self {
        return Transform { 
            position: _pos, 
            rotation: _rot 
        }
    }

    pub fn get_rotation(&self) -> Rotation {
        return self.rotation.clone()
    }

    pub fn get_position(&self) -> Position {
        return self.position.clone()
    }

    pub fn set_rotation(&mut self, _target : Rotation) {
        self.rotation = _target;
    }

    pub fn set_position(&mut self, _target : Position) {
        self.position = _target;
    }
}