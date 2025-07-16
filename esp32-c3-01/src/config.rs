// Configurações do projeto ESP32-C3 MQTT Counter

// Configurações WiFi
pub const WIFI_SSID: &str = "your_wifi_ssid";
pub const WIFI_PASSWORD: &str = "your_wifi_password";

// Configurações MQTT (futuro)
pub const MQTT_BROKER: &str = "broker.local";
pub const MQTT_PORT: u16 = 1883;
pub const MQTT_CLIENT_ID: &str = "esp32-counter";

// Tópicos MQTT
pub const MQTT_TOPIC_COUNTER: &str = "esp32/counter/value";
pub const MQTT_TOPIC_STATUS: &str = "esp32/counter/status";
pub const MQTT_TOPIC_COMMANDS: &str = "esp32/counter/cmd";

// Configurações do contador
pub const COUNTER_INTERVAL_MS: u32 = 1000;

// Configurações GPIO
pub const LED_ONBOARD_PIN: u8 = 8;
pub const LED_BAR_PINS: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

// Configurações de rede
pub const WIFI_CONNECT_TIMEOUT_MS: u32 = 10000;
pub const WIFI_RETRY_DELAY_MS: u32 = 1000;