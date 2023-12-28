use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct Circle {
    x: i32,
    y: i32,
    radius: u32,
}

impl TryFrom<i32> for Circle {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value > 0 {
            Ok( Circle{
                x: 0_i32,
                y: 0_i32,
                radius: value as u32,
            } )
        } else {
            Err(())
        }
    }
}

fn main() {
    // Try From
    assert_eq!( Circle::try_from(5_i32), Ok( Circle{ x: 0_i32, y: 0_i32, radius: 5_u32}) );
    assert_eq!( Circle::try_from(-1_i32), Err(()) );


}