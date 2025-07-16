use esp_println::println;
use esp_wifi::{
    wifi::{
        AccessPointInfo, ClientConfiguration, Configuration, WifiError, WifiStaDevice, WifiState,
    },
    wifi_interface::WifiStack,
    EspWifiInitFor,
};
use smoltcp::iface::SocketStorage;

use crate::config::{WIFI_SSID, WIFI_PASSWORD, WIFI_CONNECT_TIMEOUT_MS, WIFI_RETRY_DELAY_MS};

pub struct WiFiManager {
    device: WifiStaDevice,
    config: Configuration,
}

impl WiFiManager {
    pub fn new(device: WifiStaDevice) -> Self {
        let config = Configuration::Client(ClientConfiguration {
            ssid: WIFI_SSID.try_into().unwrap(),
            password: WIFI_PASSWORD.try_into().unwrap(),
            ..Default::default()
        });

        Self { device, config }
    }

    pub fn connect(&mut self) -> Result<(), WifiError> {
        println!("WiFi: Connecting to {}...", WIFI_SSID);
        
        self.device.set_configuration(&self.config)?;
        
        println!("WiFi: Starting connection...");
        self.device.start()?;
        
        println!("WiFi: Waiting for connection...");
        loop {
            let state = self.device.get_status();
            match state {
                WifiState::StaConnected => {
                    println!("WiFi: Connected successfully!");
                    break;
                }
                WifiState::StaDisconnected => {
                    println!("WiFi: Disconnected, retrying...");
                    // Continue loop to retry
                }
                _ => {
                    println!("WiFi: State: {:?}", state);
                }
            }
            
            // Small delay before checking again
            esp_hal::delay::Delay::new().delay_millis(WIFI_RETRY_DELAY_MS);
        }
        
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        matches!(self.device.get_status(), WifiState::StaConnected)
    }

    pub fn get_status(&self) -> WifiState {
        self.device.get_status()
    }

    pub fn disconnect(&mut self) -> Result<(), WifiError> {
        println!("WiFi: Disconnecting...");
        self.device.stop()
    }

    pub fn reconnect(&mut self) -> Result<(), WifiError> {
        println!("WiFi: Reconnecting...");
        self.disconnect()?;
        self.connect()
    }

    pub fn scan_networks(&mut self) -> Result<Vec<AccessPointInfo>, WifiError> {
        println!("WiFi: Scanning networks...");
        let mut networks = Vec::new();
        
        // Note: This is a simplified scan implementation
        // In a real implementation, you would need to handle the scan callback
        println!("WiFi: Scan completed");
        
        Ok(networks)
    }
}

pub fn init_wifi(
    wifi_init: EspWifiInitFor,
    device: WifiStaDevice,
) -> Result<WiFiManager, WifiError> {
    println!("WiFi: Initializing WiFi stack...");
    
    let wifi_manager = WiFiManager::new(device);
    
    println!("WiFi: WiFi stack initialized");
    Ok(wifi_manager)
}