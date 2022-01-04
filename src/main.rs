use std::time::Duration;

#[derive(Debug)]
enum Item {
    CntR,
    Tamb,
}

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
    let result = device_handle.read_interrupt(rusb::constants::LIBUSB_ENDPOINT_IN | 0x1, buf, Duration::new(5, 0));
    if result.is_err() {
        return
    }
    println!("{:?}", buf);
    let sum = buf[0].wrapping_add(buf[1].wrapping_add(buf[2]));
    if sum != buf[3] {
        println!("invalid");
        return
    }
    let value: u32 = (u32::try_from(buf[1]).unwrap() << 8 | buf[2] as u32).into();
    let item = item_of(*buf);
    println!("{:?} {} {}", item, tamb(value), cntr(value))
}

// http://co2meters.com/Documentation/Other/AN_RAD_0301_USB_Communications_Revised8.pdf

fn item_of(data: [u8; 8]) -> Item {
    let item: i16 = data[0].into();
    let diff_cntr = (0x50 - item).abs();
    let diff_tamb = (0x42 - item).abs();
    return if diff_cntr < diff_tamb {
        Item::CntR
    } else {
        Item::Tamb
    }
}

fn tamb(value: u32) -> f32 {
    return (value as f32) / 16.0 - 273.1
}

fn cntr(value: u32) -> u16 {
    return value as u16
}
