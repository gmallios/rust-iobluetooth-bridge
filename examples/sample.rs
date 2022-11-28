use iobluetooth_rs::bridge::scan;

fn main() {
    let res = scan();
    println!("res: {:#?}", res);
}