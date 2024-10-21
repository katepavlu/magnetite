#![warn(clippy::all, rust_2018_idioms)]

use std::f64::consts;
use std::ops::{Add, AddAssign};
// magnetic field simulator / visualiser

// used to specify location
pub struct Location {
    x: f64,
    y: f64,
    z: f64,
}

// math object used to describe an amplitude and a direction
#[derive(PartialEq, Debug, Clone)]
pub struct Vector {
    x: f64, 
    y: f64,
    z: f64
}

// phi = angle in X-Y plane, starting at X axis, positive direction
// theta = angle in Z-VECTOR plane, starting at XY plane intersection
impl Vector {
    pub fn new(x: f64, y:f64, z:f64) -> Vector {
        Vector{x, y, z}
    }

    pub fn from_polar (amplitude:f64, phi:f64, theta:f64) -> Vector {
        let x = amplitude * theta.cos() * phi.cos();
        let y = amplitude * theta.cos() * phi.sin();
        let z = amplitude * theta.sin();

        Vector{x, y, z}
    }

    pub fn abs(&self) -> f64 {
        ( self.x.powi(2) + self.y.powi(2) + self.z.powi(2) ).sqrt()
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

pub struct CurrentSgmt {
    loc: Location,
    vec: Vector,
    len: f64,
}


//evaluate integral
//of 1/(x^2 - ax + b)^3/2
//at x
//(C not considered)
fn intres(x:f64, a:f64, b:f64) -> f64 {
    2.0 * (2.0*x - a) / ( (4.0*b - a*a) * ((x*(x - a) + b).sqrt()) )
}

impl CurrentSgmt {
    pub fn induced_field (&self, pnt: &Location) -> Vector {

        let r0 = Vector{ 
            x: -self.loc.x + pnt.x,
            y: -self.loc.y + pnt.y,
            z: -self.loc.z + pnt.z,
        };

        let a = 2.0 * 
            (r0.x * self.vec.x + r0.y * self.vec.y + r0.z * self.vec.z) /
            self.vec.abs();

        let b = r0.x.powi(2) + r0.y.powi(2) + r0.z.powi(2);

        println!("A is {a}, B is {b}");

        let mul = 1e-7 * (intres(self.len, a, b) - intres(0.0, a, b));

        Vector{ 
            x: mul * ( self.vec.y * r0.z - self.vec.z * r0.y ),
            y: mul * ( self.vec.z * r0.x - self.vec.x * r0.z ),
            z: mul * ( self.vec.x * r0.y - self.vec.y * r0.x ),
        }
    }
}

pub struct Field {
    currents: Vec<CurrentSgmt>,
}

impl Field {
    pub fn new() -> Field {
        Field {currents: Vec::new()}
    }
    pub fn eval_at_pnt (&self, loc:Location) -> Vector {
        let mut res = Vector {x:0.0, y:0.0, z:0.0};
        for cur in &self.currents {
            res += cur.induced_field(&loc);
        }
        res
    }
}

mod app;
pub use app::TemplateApp;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn polar() {
        assert_eq!(
            Vector{x:1.0, y:0.0, z:0.0},
            Vector::from_polar(1.0, 0.0, 0.0),
            );
        assert!(comp_vectors(
            &Vector{x: 6.0_f64.sqrt()/4.0, y:6.0_f64.sqrt()/4.0, z:0.5},
            &Vector::from_polar(1.0, consts::PI/4.0 , consts::PI/6.0))
            );
    }

    #[test]
    fn add() {
        assert_eq!(
            Vector{x:2.0, y:3.0, z:4.0},
            Vector{x:1.0, y:2.0, z:3.0} + Vector{x:1.0, y:1.0, z:1.0},
            );
    }
    #[test]
    fn add_eq() {
        let mut v1 = Vector{x:1.0, y:1.0, z:1.0};
        let v2 = Vector{x:10.0, y:20.0, z:30.0};
        v1 += v2;
        assert_eq!(
            Vector{x:11.0, y:21.0, z:31.0},
            v1
            );
    }

    fn comp_vectors(v1:&Vector, v2:&Vector) -> bool {
        (v1.x - v2.x).abs() < 1e-5 &&
        (v1.y - v2.y).abs() < 1e-5 &&
        (v1.z - v2.z).abs() < 1e-5
    }

    #[test]
    fn induced() {
        let sgmt = CurrentSgmt{
            loc: Location{ x: -500.0, y:0.0, z:0.0},
            vec: Vector::new(1.0, 0.0, 0.0),
            len: 1000.0,
            };
        // calculate by infinite wire approximaton
        let v1 = Vector::new(0.0, 0.0, 4.0 * consts::PI * 1e-7 / (2.0 * consts::PI));
        // calculate by Biot-Savart
        let v2 = sgmt.induced_field(&Location{x:0.0, y:1.0, z:0.0});
        assert!(comp_vectors(&v1, &v2));
    }
    
    #[test]
    fn more_pnts() {
        let mut b_field = Field{currents: Vec::new()};
        b_field.currents.push(
            CurrentSgmt{
                loc: Location{ x:-500.0, y:-1.0, z:0.0},
                vec: Vector::new(1.0, 0.0, 0.0),
                len: 100.0,
            });
        b_field.currents.push(
            CurrentSgmt{
                loc: Location{ x:500.0, y:1.0, z:0.0},
                vec: Vector::new(-1.0, 0.0, 0.0),
                len: 100.0,
            });
        assert!(comp_vectors(
                &b_field.eval_at_pnt(Location{x:0.0, y:0.0, z:0.0}),
                &Vector::new(0.0, 0.0, 0.0)
                ));
    }

}
