# Configuração WiFi - ESP32-C3 MQTT Counter

## Configuração Básica

Para configurar o WiFi, edite o arquivo `src/config.rs`:

```rust
// Configurações WiFi
pub const WIFI_SSID: &str = "SEU_WIFI_SSID";
pub const WIFI_PASSWORD: &str = "SUA_SENHA_WIFI";
```

## Indicadores LED

O LED onboard (GPIO8) indica o status da conexão WiFi:

### Padrões de LED
- **Conexão bem-sucedida**: 10 piscadas rápidas (100ms)
- **Falha na conexão**: 5 piscadas lentas (1000ms)
- **Conectado**: Piscada lenta (1.8s ligado, 0.2s desligado)
- **Desconectado**: Piscada rápida (100ms) + tentativa de reconexão

## Funcionalidades Implementadas

### WiFiManager
- **Conexão automática** ao access point
- **Reconexão automática** em caso de perda de sinal
- **Monitoramento de status** em tempo real
- **Configuração centralizada** via config.rs

### Timeouts e Delays
- **Timeout de conexão**: 10 segundos (configurável)
- **Delay entre tentativas**: 1 segundo
- **Intervalo de reconexão**: 10 segundos

## Testando a Conectividade

### 1. Configurar credenciais WiFi
```rust
// Em src/config.rs
pub const WIFI_SSID: &str = "MinhaRedeWiFi";
pub const WIFI_PASSWORD: &str = "MinhasenhaWiFi123";
```

### 2. Compilar e flashear
```bash
cargo build
cargo run
```

### 3. Monitorar logs
```bash
# No terminal do ESP32-C3
WiFi: Connecting to MinhaRedeWiFi...
WiFi: Starting connection...
WiFi: Waiting for connection...
WiFi: Connected successfully!
```

## Troubleshooting

### Problemas Comuns

**1. Falha na conexão**
- Verifique SSID e senha
- Certifique-se que o WiFi está no alcance
- Verifique se a rede aceita ESP32-C3

**2. Desconexões frequentes**
- Verifique qualidade do sinal
- Ajuste `WIFI_RETRY_DELAY_MS` em config.rs
- Considere usar IP estático

**3. Não consegue reconectar**
- Reinicie o ESP32-C3
- Verifique se o roteador não está bloqueando o dispositivo

### Logs de Debug
```
WiFi: Initializing WiFi stack...
WiFi: WiFi stack initialized
WiFi: Starting connection process...
WiFi: Connecting to MinhaRedeWiFi...
WiFi: Starting connection...
WiFi: Waiting for connection...
WiFi: State: StaDisconnected
WiFi: State: StaConnecting
WiFi: Connected successfully!
```

## Configurações Avançadas

### Timeout Personalizado
```rust
// Em src/config.rs
pub const WIFI_CONNECT_TIMEOUT_MS: u32 = 15000; // 15 segundos
pub const WIFI_RETRY_DELAY_MS: u32 = 2000;      // 2 segundos
```

### IP Estático (Futuro)
```rust
// Planejado para próximas versões
pub const WIFI_USE_STATIC_IP: bool = true;
pub const WIFI_STATIC_IP: &str = "192.168.1.100";
pub const WIFI_GATEWAY: &str = "192.168.1.1";
pub const WIFI_SUBNET: &str = "255.255.255.0";
```

## Próximos Passos

1. **Implementar MQTT** - Usar a conexão WiFi para comunicação MQTT
2. **Adicionar NTP** - Sincronização de tempo via rede
3. **Web interface** - Configuração via navegador
4. **OTA updates** - Atualizações over-the-air

---

*Para mais informações, consulte o arquivo CLAUDE.md*