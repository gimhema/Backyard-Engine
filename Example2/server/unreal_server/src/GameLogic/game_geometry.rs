use std::fmt::Pointer;



pub struct Rotation
{
    q_x : f64,
    q_y : f64,
    q_z : f64,
    f_w : f64
}

pub struct Position
{
    x : f64,
    y : f64,
    z : f64
}

pub struct Transform
{
    rotation : Rotation,
    position : Position
}

