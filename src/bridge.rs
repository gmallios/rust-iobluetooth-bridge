use std::sync::{Mutex, Arc};

lazy_static! {
    static ref INTERNAL_DEVICE_LIST: Arc<Mutex<Vec<ffi::BtDevice>>> = Arc::new(Mutex::new(Vec::new()));
}

#[swift_bridge::bridge]
pub mod ffi {

    #[swift_bridge::bridge(swift_repr = "struct")]
    struct BtDevice {
        name: String,
        addr: Vec<u8>,
    }

    extern "Swift" {
        fn scan();
    }

    extern "Rust" {
        fn create_string(str: &str) -> String;
        fn add_to_device_list(device: BtDevice);
    }
}

fn create_string(str: &str) -> String {
    str.to_string()
}

pub fn add_to_device_list(device: ffi::BtDevice) {
    let mut list = INTERNAL_DEVICE_LIST.lock().unwrap();
    list.push(device);
}

#[derive(Debug)]
pub struct BthDevice {
    pub name: String,
    pub addr: Vec<u8>,
}

impl From<&ffi::BtDevice> for BthDevice {
    fn from(device: &ffi::BtDevice) -> Self {
        Self {
            name: device.name.clone(),
            addr: device.addr.clone(),
        }
    }
}

pub fn scan() -> Vec<BthDevice> {
    ffi::scan();
    let mut res = Vec::new();
    for(_i, device) in INTERNAL_DEVICE_LIST.lock().unwrap().iter().enumerate() {
        res.push(BthDevice::from(device));
    }
    res
}

