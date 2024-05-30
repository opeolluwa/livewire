use livewire::{WifiHotspotConfig, WifiHotspotConfigBuilder};

fn main() {

let available_networks = WifiHotspotConfig::scan();

    println!("Hello, world!, {:#?}", available_networks);
}
