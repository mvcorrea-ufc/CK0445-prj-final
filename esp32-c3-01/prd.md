# PRD - ESP32-C3 MQTT Counter Project

## 1. Visão Geral do Produto

### 1.1 Descrição
Sistema embarcado baseado em ESP32-C3 que implementa um contador de 8 bits com transmissão MQTT e visualização em barra de LEDs, utilizando apenas esp-hal sem RTOS.

### 1.2 Objetivos
- Demonstrar comunicação MQTT em dispositivos embarcados
- Implementar sistema de contagem com feedback visual
- Criar arquitetura modular e escalável
- Estabelecer base para sistemas IoT distribuídos

### 1.3 Escopo
- **Incluído**: Contador, WiFi, MQTT, barra de LEDs, controle remoto
- **Excluído**: Interface web, armazenamento persistente, múltiplos sensores

## 2. Especificações Técnicas

### 2.1 Hardware
- **Microcontrolador**: ESP32-C3 (RISC-V, 160MHz)
- **Memória**: 400KB SRAM, 4MB Flash
- **Conectividade**: WiFi 802.11 b/g/n
- **LEDs**: 8 LEDs para visualização binária
- **Alimentação**: USB ou 3.3V externa

### 2.2 Software
- **Linguagem**: Rust
- **HAL**: esp-hal v0.23.1
- **Target**: riscv32imc-unknown-none-elf
- **Sem RTOS**: Implementação bare-metal

### 2.3 Comunicação
- **Protocolo**: MQTT v3.1.1
- **Broker**: Mosquitto
- **Tópicos**: 
  - `esp32/counter/value` - Valor atual do contador
  - `esp32/counter/status` - Status do sistema
  - `esp32/counter/cmd` - Comandos remotos

## 3. Funcionalidades

### 3.1 Core Features (MVP)
- **Contador 8-bit**: 0-255 com incremento automático (1s)
- **WiFi**: Conexão automática a access point
- **MQTT**: Envio periódico do valor do contador
- **Barra LEDs**: Visualização binária do contador
- **Logging**: Saída via UART para debug

### 3.2 Funcionalidades Avançadas
- **Controle Remoto**: Comandos via MQTT (reset, pause, intervalo)
- **Reconexão**: Automática em caso de perda de conectividade
- **Heartbeat**: Envio periódico de status
- **Configuração**: Parâmetros via MQTT

### 3.3 Funcionalidades Futuras
- **Múltiplos Contadores**: Contadores independentes
- **Sensores**: Integração com sensores externos
- **OTA Updates**: Atualização remota de firmware
- **Configuração Web**: Interface de configuração

## 4. Arquitetura

### 4.1 Estrutura Modular
```
src/
├── main.rs              # Ponto de entrada e loop principal
├── counter.rs           # Módulo do contador 8-bit
├── wifi.rs              # Gestão da conexão WiFi
├── mqtt.rs              # Cliente MQTT
├── led_bar.rs           # Controle da barra de LEDs
├── config.rs            # Configurações do sistema
└── utils.rs             # Utilitários gerais
```

### 4.2 Fluxo de Execução
1. **Inicialização**: Configuração de periféricos
2. **WiFi**: Conexão ao access point
3. **MQTT**: Conexão ao broker
4. **Loop Principal**: 
   - Incrementar contador
   - Atualizar LEDs
   - Enviar via MQTT
   - Processar comandos
   - Delay de 1s

### 4.3 Gestão de Estados
- **DISCONNECTED**: Sem conectividade
- **CONNECTING**: Conectando WiFi/MQTT
- **CONNECTED**: Operação normal
- **ERROR**: Estado de erro

## 5. Interface e Experiência

### 5.1 Feedback Visual
- **LEDs**: Representação binária do contador (8 LEDs)
- **LED Status**: Indicador de conectividade
- **Padrões**: Diferentes sequências para diferentes estados

### 5.2 Comandos MQTT
```json
{
  "cmd": "reset",
  "timestamp": "2025-07-15T10:00:00Z"
}

{
  "cmd": "set_interval",
  "value": 2000,
  "timestamp": "2025-07-15T10:00:00Z"
}

{
  "cmd": "pause",
  "timestamp": "2025-07-15T10:00:00Z"
}
```

### 5.3 Formato de Dados
```json
{
  "counter": 42,
  "uptime": 3600,
  "wifi_rssi": -45,
  "heap_free": 180000,
  "timestamp": "2025-07-15T10:00:00Z"
}
```

## 6. Configuração e Deployment

### 6.1 Ambiente de Desenvolvimento
- **Container**: Docker/Podman com SSH
- **IDE**: VSCode com Remote SSH
- **Ferramentas**: Rust, espflash, esptool

### 6.2 Configuração WiFi
```rust
const WIFI_SSID: &str = "your_network";
const WIFI_PASSWORD: &str = "your_password";
```

### 6.3 Configuração MQTT
```rust
const MQTT_BROKER: &str = "broker.local";
const MQTT_PORT: u16 = 1883;
const MQTT_CLIENT_ID: &str = "esp32-counter";
```

## 7. Testes e Validação

### 7.1 Testes Unitários
- Módulo contador (overflow, reset)
- Formatação de mensagens MQTT
- Controle de LEDs

### 7.2 Testes de Integração
- Conectividade WiFi
- Comunicação MQTT
- Sincronização LED-contador

### 7.3 Testes de Sistema
- Operação contínua (24h)
- Recuperação de falhas
- Desempenho de conectividade

## 8. Métricas e Monitoramento

### 8.1 KPIs
- **Uptime**: > 99% em 24h
- **Latência MQTT**: < 100ms
- **Consumo de Memória**: < 200KB
- **Reconexão**: < 30s

### 8.2 Logs
- Eventos de conectividade
- Comandos recebidos
- Erros e exceções
- Métricas de performance

## 9. Roadmap

### 9.1 Versão 1.0 (MVP)
- Contador básico com MQTT
- Barra de LEDs
- Conectividade WiFi estável

### 9.2 Versão 1.1
- Controle remoto via MQTT
- Configuração dinâmica
- Melhor tratamento de erros

### 9.3 Versão 2.0
- Múltiplos contadores
- Interface web
- Armazenamento de configuração

## 10. Riscos e Mitigações

### 10.1 Riscos Técnicos
- **Instabilidade WiFi**: Implementar reconexão robusta
- **Overflow de Memória**: Monitoramento e otimização
- **Latência MQTT**: Configuração adequada de timeouts

### 10.2 Riscos de Desenvolvimento
- **Complexidade esp-hal**: Documentação e exemplos
- **Debugging**: Ferramentas apropriadas
- **Testes**: Ambiente de teste automatizado

---

*Documento versão 1.0 - 2025-07-15*  
*Próxima revisão: Após implementação do MVP*