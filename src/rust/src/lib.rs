use extendr_api::{
    graphics::{DevDesc, DeviceDescriptor, DeviceDriver},
    prelude::*,
};

#[allow(dead_code)]
struct YadnDevice<'a> {
    welcome_message: &'a str,
    show_message: bool,
}

impl<'a> DeviceDriver for YadnDevice<'a> {
    fn activate(&mut self, _dd: DevDesc) {
        let welcome_message = self.welcome_message;

        if self.show_message {
            rprintln!("🎉🍕🍰📺🍓✨🍣🐈🎿🎉🍕🍰📺🍓✨🍣🐈🎿");
            rprintln!("");
            rprintln!("   {welcome_message}   ");
            rprintln!("");
            rprintln!("🎉🍕🍰📺🍓✨🍣🐈🎿🎉🍕🍰📺🍓✨🍣🐈🎿");
        }
    }
}

/// A graphic device that does nothing
///
/// @param welcome_message A warm message to welcome you.
/// @param show_message Whether to show the welcome message.
/// @export
#[extendr]
fn yadngd(welcome_message: String, show_message: bool) {
    let device_driver = YadnDevice {
        welcome_message: welcome_message.as_str(),
        show_message,
    };

    let device_descriptor = DeviceDescriptor::new();

    device_driver.create_device::<YadnDevice>(device_descriptor, "yadndgd");
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod yadngd;
    fn yadngd;
}
