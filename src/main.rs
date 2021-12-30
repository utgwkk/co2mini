use std::time::Duration;

fn main() {
    let vid = 0x04d9;
    let pid = 0xa052;
    let device_handle = rusb::open_device_with_vid_pid(vid, pid);
    if device_handle.is_none() {
        return
    }
    let mut device_handle = device_handle.unwrap();
    let result = device_handle.claim_interface(0);
    if result.is_err() {
        return
    }

    let buf = &mut [0; 8];
    let result = device_handle.read_bulk(rusb::constants::LIBUSB_ENDPOINT_IN | 0x1, buf, Duration::new(5, 0));
    if result.is_err() {
        return
    }
    println!("{:?}", buf);
    println!("{} {}", tamb(*buf), cntr(*buf))
}

// http://co2meters.com/Documentation/Other/AN_RAD_0301_USB_Communications_Revised8.pdf

fn tamb(data: [u8; 8]) -> f32 {
    let msb: f32 = data[1].into();
    let lsb: f32 = data[2].into();
    return msb + lsb * 0.01
}

fn cntr(data: [u8; 8]) -> u16 {
    let msb: u16 = data[1].into();
    let lsb: u16 = data[2].into();
    return msb * 100 + lsb;
}
