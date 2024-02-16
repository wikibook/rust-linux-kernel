struct Adaptee {}

impl Adaptee {
    fn new() -> Adaptee {
        Adaptee {}
    }

    fn vendor_specific_api(&self) {
        println!("벤더가 정의한 API")
    }
}

struct Adapter {}

impl Adapter {
    fn new() -> Adapter {
        Adapter {}
    }
    
    // 우리가 정의한 API
    fn call_api(&self) {
        Adaptee::new().vendor_specific_api();
    }
}

fn main() {
    let adapter = Adapter::new();
    adapter.call_api();
}