/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod deserialize;
mod serialize;

pub use deserialize::{Deserializable, Deserializer};
pub use serialize::{Serializable, Serializer};

#[derive(Debug)]
pub enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Texting,
}

#[derive(Debug)]
pub struct MathRequest {
    pub id: u32,
    pub operation: Operation,
    pub a: f64,
    pub b: f64,
    pub s: String,
}

#[derive(Debug)]
pub struct MathResult {
    pub id: u32,
    pub res: f64,
    pub text: String,
}

impl MathRequest {
    pub fn add(a: f64, b: f64) -> MathRequest {
        MathRequest {
            id: rand::random(),
            operation: Operation::Addition,
            a,
            b,
            s: "".into(),
        }
    }

    pub fn subtract(a: f64, b: f64) -> MathRequest {
        MathRequest {
            id: rand::random(),
            operation: Operation::Subtraction,
            a,
            b,
            s: "".into(),
        }
    }

    pub fn multiply(a: f64, b: f64) -> MathRequest {
        MathRequest {
            id: rand::random(),
            operation: Operation::Multiplication,
            a,
            b,
            s: "".into(),
        }
    }

    pub fn divide(a: f64, b: f64) -> MathRequest {
        MathRequest {
            id: rand::random(),
            operation: Operation::Division,
            a,
            b,
            s: "".into(),
        }
    }

    pub fn text(s: String) -> MathRequest {
        MathRequest {
            id: rand::random(),
            operation: Operation::Texting,
            a: 0.0,
            b: 0.0,
            s,
        }
    }
}
