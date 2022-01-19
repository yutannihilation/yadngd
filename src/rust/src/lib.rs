use extendr_api::{
    graphics::{device_descriptor::DeviceDescriptor, device_driver::DeviceDriver, DevDesc},
    prelude::*,
};

struct YadnDevice<'a> {
    welcome_message: &'a str,
}

impl<'a> DeviceDriver for YadnDevice<'a> {
    fn activate(&mut self, _dd: DevDesc) {
        let welcome_message = self.welcome_message;

        rprintln!("🎉🍕🍰📺🍓✨🍣🐈🎿🎉🍕🍰📺🍓✨🍣🐈🎿");
        rprintln!("");
        rprintln!("   {welcome_message}   ");
        rprintln!("");
        rprintln!("🎉🍕🍰📺🍓✨🍣🐈🎿🎉🍕🍰📺🍓✨🍣🐈🎿");
    }
}

/// Return string `"Hello world!"` to R.
///
/// @export
#[extendr]
fn yadngd(welcome_message: String) -> i32 {
    let device_driver = YadnDevice {
        welcome_message: welcome_message.as_str(),
    };

    let device_descriptor = DeviceDescriptor::new();
    let device = device_driver.create_device::<YadnDevice>(device_descriptor, "yadndgd");
    device.device_number()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod yadngd;
    fn yadngd;
}
