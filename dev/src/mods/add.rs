// add1 Option
pub fn add1(a: Option<i32>, b: Option<i32>) -> i32 {
    a.unwrap_or(1) + b.unwrap_or(2)
}

// add2 macro
pub fn add2(a: i32, b: i32) -> i32 {
    a + b
}
#[macro_export]
macro_rules! add2 { 
    ($a: expr, $b: expr) => {
        add2($a, $b)
    };

    ($a: expr) => {
        add2($a, 2)
    };
    () => {
        add2(1, 2)
    };
}

// add3 From/Into trait
pub struct AddArgs {
    a: i32,
    b: i32,
}

impl Default for AddArgs {
    fn default() -> Self {
        AddArgs { a: 1, b: 2 }
    }
}

impl From<()> for AddArgs {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl From<i32> for AddArgs {
    fn from(a: i32) -> Self {
        Self {
            a,
            ..Self::default()
        }
    }
}

impl From<(i32, i32)> for AddArgs {
    fn from((a, b): (i32, i32)) -> Self {
        Self { a, b }
    }
}

pub fn add3<A>(arg_like: A) -> i32
    where
        A: Into<AddArgs>,
{
    let args = arg_like.into();
    args.a + args.b
}
