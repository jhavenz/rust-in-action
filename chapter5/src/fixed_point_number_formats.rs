// In addition to representing decimal numbers with floating-point formats,
// fixed point is also available. These can be useful for representing
// fractions and are an option for performing calculations on CPUs without
// a floating point unit (FPU), such as microcontrollers. Unlike
// floating-point numbers, the decimal place does not move to dynamically
// accommodate different ranges. In our case, we’ll be using a fixed-point
// number format to compactly represent values between –1..=1. *Although it
// loses accuracy, it saves significant space.
// Note on '*':
// This practice is known as 'quantizing the model' in the machine learning community.

// The 'Q format' is a fixed-point number format that uses a single byte. It was created
// by Texas Instruments for embedded computing devices. The specific version of the Q
// format that we will implement is called Q7.
// Note:
// Q, often written as ℚ (this style is called blackboard bold), is the mathematical
// symbol for the so-called rational numbers. Rational numbers are numbers that can
// be represented as a fraction of two integers, such as 1/3.

/// known as a 'tuple struct'. Accessors look like: `Q7(1.42).0` -> 1.42.
/// Q7's most important role is to convert to and from floating-point types.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Q7(i8);

impl From<f64> for Q7 {
    fn from (n: f64) -> Self {
        // assert!(n >= -1.0);
        // assert!(n <= 1.0);
        if n >= 1.0 {
            Q7(127)
        } else if n <= -1.0 {
            Q7(-128)
        } else {
            Q7((n * 128.0) as i8)
        }
    }
}

impl From<Q7> for f64 {
    fn from(n: Q7) -> f64 {
        (n.0 as f64) * 2_f64.powf(-7.0)
    }
}

impl From<f32> for Q7 {
    fn from (n: f32) -> Self {
        // Converting from a smaller type to a larger type is always safe.
        Q7::from(n as f64)
    }
}

impl From<Q7> for f32 {
    fn from(n: Q7) -> f32 {
        // Converting from a (potentially) larger type to a smaller type is NOT always safe.
        // In this application, this risk does not apply as we'll only have numbers between –1 and 1 to convert from.
        f64::from(n) as f32
    }
}

// Generating f32 values in interval [0,1] from a u8 value.
pub fn mock_rand(n: u8) -> f32 {
    let base: u32 = 0b0_01111110_00000000000000000000000;

    let large_n = (n as u32) << 15;

    let f32_bits = base | large_n;

    let m = f32::from_bits(f32_bits);

    2.0 * ( m - 0.5 )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn out_of_bounds() {
        assert_eq!(Q7::from(10.), Q7::from(1.));
        assert_eq!(Q7::from(-10.), Q7::from(-1.));
    }

    #[test]
    fn f32_to_q7() {
        let n1: f32 = 0.7;
        let q1 = Q7::from(n1);

        let n2 = -0.4;
        let q2 = Q7::from(n2);

        let n3 = 123.0;
        let q3 = Q7::from(n3);

        assert_eq!(q1, Q7(89));
        assert_eq!(q2, Q7(-51));
        assert_eq!(q3, Q7(127));
    }

    #[test]
    fn q7_to_f32() {
        let q1 = Q7::from(0.7);
        let n1 = f32::from(q1);
        assert_eq!(n1, 0.6953125);

        let q2 = Q7::from(n1);
        let n2 = f32::from(q2);
        assert_eq!(n1, n2);
    }
}
