use macpow::smc::SmcConnection;

fn main() {
    let mut smc = SmcConnection::open().expect("SMC open");
    let keys = [
        "wiPm", "PWFI", "wiTx", "wiRx", "wiSt", "wiRS", "wiNS", "PDBR", "PDTR", "PSTR", "PPBR",
        "PHPC", "PSVR",
    ];
    for key in &keys {
        match smc.read_f32(key) {
            Ok(v) => println!("{}: {:>8.4} W", key, v),
            Err(_) => {}
        }
    }
}
