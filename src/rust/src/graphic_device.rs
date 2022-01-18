use extendr_api::{
    graphics::{device_descriptor::DeviceDescriptor, device_driver::DeviceDriver, DevDesc},
    prelude::*,
};

// https://github.com/coolbutuseless/devout/blob/7bd337aab8f37a05db36afc5ff0ad643bc518449/src/rdevice.cpp#L1711-L1730

struct YadnDevice {}

impl DeviceDriver for YadnDevice {
    fn activate(_: DevDesc) {
        rprintln!("🎉🍕🍰📺🍓✨🍣🐈🎿🎉🍕🍰📺🍓✨🍣🐈🎿");
        rprintln!("");
        rprintln!("   ◆祝◆ device activated!!! ◆祝◆   ");
        rprintln!("");
        rprintln!("🎉🍕🍰📺🍓✨🍣🐈🎿🎉🍕🍰📺🍓✨🍣🐈🎿");
    }
}

pub fn make_graphic_device() -> i32 {
    let device_driver = YadnDevice {};
    let device_descriptor = DeviceDescriptor::new();
    let device = device_driver.create_device::<YadnDevice>(device_descriptor, "yadndgd");
    device.device_number()
}
