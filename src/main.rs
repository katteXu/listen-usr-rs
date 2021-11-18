use rusb::{Context, Device, HotplugBuilder, UsbContext};

struct HotPlugHandler;

impl<T: UsbContext> rusb::Hotplug<T> for HotPlugHandler {
    fn device_arrived(&mut self, device: Device<T>) {
        let desc = device.device_descriptor().unwrap();
        println!("<--接入--> {:?}", device);
    }

    fn device_left(&mut self, device: Device<T>) {
        println!("<--离开-->{:?}", device);
    }
}

impl Drop for HotPlugHandler {
    fn drop(&mut self) {
        // println!("---一轮结束---");
        listen();
    }
}

fn main() {
    listen();
}

fn listen() -> rusb::Result<()> {
    if rusb::has_hotplug() {
        let context = Context::new()?;

        let mut reg = Some(
            HotplugBuilder::new()
                .enumerate(false)
                .register(&context, Box::new(HotPlugHandler {}))?,
        );
        loop {
            context.handle_events(None).unwrap();
            if let Some(reg) = reg.take() {
                context.unregister_callback(reg);
                break;
            }
        }
        Ok(())
    } else {
        eprint!("libusb hotplug api unsupported");
        Ok(())
    }
}
