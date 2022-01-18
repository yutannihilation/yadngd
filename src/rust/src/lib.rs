use extendr_api::prelude::*;

mod graphic_device;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn yadngd(welcome_message: String) -> i32 {
    graphic_device::make_graphic_device(welcome_message.as_str())
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod yadngd;
    fn yadngd;
}
