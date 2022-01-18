use extendr_api::{
    graphics::{device_descriptor::DeviceDescriptor, device_driver::DeviceDriver, DevDesc},
    prelude::*,
};

// https://github.com/coolbutuseless/devout/blob/7bd337aab8f37a05db36afc5ff0ad643bc518449/src/rdevice.cpp#L1711-L1730

struct YadnDevice<'a> {
    welcome_message: &'a str,
}

impl<'a> DeviceDriver for YadnDevice<'a> {
    fn activate(self, _dd: DevDesc) {
        let welcome_message = self.welcome_message;

        rprintln!("🎉🍕🍰📺🍓✨🍣🐈🎿🎉🍕🍰📺🍓✨🍣🐈🎿");
        rprintln!("");
        rprintln!("   {welcome_message}   ");
        rprintln!("");
        rprintln!("🎉🍕🍰📺🍓✨🍣🐈🎿🎉🍕🍰📺🍓✨🍣🐈🎿");
    }
}

pub fn make_graphic_device(welcome_message: &str) -> i32 {
    let device_driver = YadnDevice { welcome_message };

    let device_descriptor = DeviceDescriptor::new();
    let device = device_driver.create_device::<YadnDevice>(device_descriptor, "yadndgd");
    device.device_number()
}
