import IOBluetooth
import Cocoa

var devices: [IOBluetoothDevice] = []

class BlueDelegate : NSObject, IOBluetoothDeviceInquiryDelegate {
    func deviceInquiryStarted(_ sender: IOBluetoothDeviceInquiry) {
        print("Inquiry Started...")
    }

    func deviceInquiryDeviceFound(_ sender: IOBluetoothDeviceInquiry, device: IOBluetoothDevice) {
        let name: RustString = create_string(device.name!)
        let addr = RustVec<UInt8>()
        addr.push(value: device.getAddress().pointee.data.0)
        add_to_device_list(BtDevice(name: name, addr: addr))
        devices.append(device)
    }
    
    func deviceInquiryComplete(_ sender: IOBluetoothDeviceInquiry!, error: IOReturn, aborted: Bool) {
        CFRunLoopStop(CFRunLoopGetCurrent())
    }
}


func scan() {
    let delegate = BlueDelegate()
    let ibdi = IOBluetoothDeviceInquiry(delegate: delegate)
    ibdi!.start()
    CFRunLoopRun()
    sleep(1)
    ibdi!.stop()
}