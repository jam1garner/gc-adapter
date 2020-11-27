use rusb::{Context, Device, DeviceHandle, Result, UsbContext};
use std::time::Duration;

// device uid pid are picked directly form `lsusb` result
const VID: u16 = 0x057e;
const PID: u16 = 0x0337;

fn main() -> Result<()> {
    let mut context = Context::new()?;
    let (mut device, mut handle) =
        open_device(&mut context, VID, PID).expect("Did not find USB device");

    print_device_info(&mut handle)?;

    dbg!(device.device_descriptor().unwrap().usb_version());
    dbg!(device.device_descriptor().unwrap());

    let endpoints = find_readable_endpoints(&mut device)?;
    dbg!(&endpoints);
    let endpoint = endpoints
        .get(0)
        .expect("No Configurable endpoint found on device");

    dbg!(&endpoint);

    let has_kernel_driver = match handle.kernel_driver_active(endpoint.iface) {
        Ok(true) => {
            handle.detach_kernel_driver(endpoint.iface)?;
            true
        }
        _ => false,
    };
    println!("has kernel driver? {}", has_kernel_driver);

    // claim and configure device
    configure_endpoint(&mut handle, &endpoint)?;
    // control device here

    dbg!(handle.write_interrupt(2, &[0x13], Duration::from_secs(0)));

    let mut buf = [0u8; 37];
    dbg!(handle.read_interrupt(0x81, &mut buf, Duration::from_secs(0)));

    // cleanup after use
    handle.release_interface(endpoint.iface)?;
    if has_kernel_driver {
        handle.attach_kernel_driver(endpoint.iface)?;
    }
    Ok(())
}

fn open_device<T: UsbContext>(
    context: &mut T,
    vid: u16,
    pid: u16,
) -> Option<(Device<T>, DeviceHandle<T>)> {
    let devices = match context.devices() {
        Ok(d) => d,
        Err(_) => return None,
    };

    for device in devices.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
            match device.open() {
                Ok(handle) => return Some((device, handle)),
                Err(_) => continue,
            }
        }
    }

    None
}

fn print_device_info<T: UsbContext>(handle: &mut DeviceHandle<T>) -> Result<()> {
    let device_desc = handle.device().device_descriptor()?;
    let timeout = Duration::from_secs(1);
    let languages = handle.read_languages(timeout)?;

    println!("Active configuration: {}", handle.active_configuration()?);

    if !languages.is_empty() {
        let language = languages[0];
        println!("Language: {:?}", language);

        println!(
            "Manufacturer: {}",
            handle
                .read_manufacturer_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );
        println!(
            "Product: {}",
            handle
                .read_product_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );
        println!(
            "Serial Number: {}",
            handle
                .read_serial_number_string(language, &device_desc, timeout)
                .unwrap_or("Not Found".to_string())
        );
    }
    Ok(())
}

#[derive(Debug)]
struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
    address: u8,
}

// returns all readable endpoints for given usb device and descriptor
fn find_readable_endpoints<T: UsbContext>(device: &mut Device<T>) -> Result<Vec<Endpoint>> {
    let device_desc = device.device_descriptor()?;
    let mut endpoints = vec![];
    for n in 0..device_desc.num_configurations() {
        let config_desc = match device.config_descriptor(n) {
            Ok(c) => c,
            Err(_) => continue,
        };
        // println!("{:#?}", config_desc);
        for interface in config_desc.interfaces() {
            for interface_desc in interface.descriptors() {
                // println!("{:#?}", interface_desc);
                for endpoint_desc in interface_desc.endpoint_descriptors() {
                    // println!("{:#?}", endpoint_desc);
                    endpoints.push(Endpoint {
                        config: config_desc.number(),
                        iface: interface_desc.interface_number(),
                        setting: interface_desc.setting_number(),
                        address: endpoint_desc.address(),
                    });
                }
            }
        }
    }

    Ok(endpoints)
}

fn configure_endpoint<T: UsbContext>(
    handle: &mut DeviceHandle<T>,
    endpoint: &Endpoint,
) -> Result<()> {
    //handle.set_active_configuration(endpoint.config).unwrap();
    handle.claim_interface(endpoint.iface).unwrap();
    //handle.set_alternate_setting(endpoint.iface, endpoint.setting)?;

    Ok(())
}
