use super::WifiHotspotConfig;
use super::WifiHotspotConfigBuilder;


use wifiscanner;
use wifiscanner::Wifi;

impl WifiHotspotConfigBuilder for WifiHotspotConfig {
    fn create() -> Self {
        todo!()
    }

    fn connect() -> bool {
        todo!()
    }

    fn stop() {
        todo!()
    }

    fn refresh() {
        todo!()
    }

    fn scan() -> Result<Vec<Wifi>, String> {
        let available_network = wifiscanner::scan().ok();
        if available_network.is_none() {
            return  Err("Couldn't retrieve available networks".to_string());
        }
        Ok(available_network.unwrap())
    }
}
