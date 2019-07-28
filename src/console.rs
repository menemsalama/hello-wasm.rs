#[macro_export]
#[cfg(target_arch = "wasm32")]
macro_rules! log {
    ($arg:expr) => {
        js! {
            console.log(@{$arg})
        }
    };
    ($fmt:expr, $($y:expr),+) => {
        let msg = format!($fmt, $($y),+);
        js! {
            console.log(@{msg})
        }
    };
}

#[macro_export]
#[cfg(not(target_arch = "wasm32"))]
macro_rules! log {
    ($arg:expr) => {
        println!("{}", $arg)
    };
    ($fmt:expr, $($y:expr),+) => {
        println!($fmt, $($y),+)
    };
}
