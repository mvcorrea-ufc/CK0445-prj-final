# CLAUDE.md - ESP32-C3 MQTT Counter Project

## Contexto do Projeto

Este projeto implementa um contador MQTT para ESP32-C3 utilizando apenas esp-hal (sem RTOS). O desenvolvimento é feito em container Docker/Podman com acesso via SSH.

## Estrutura do Projeto

```
esp32-c3-01/
├── src/
│   └── main.rs              # Código principal - exemplo blink
├── .cargo/
│   └── config.toml          # Configurações do Rust para ESP32-C3
├── .vscode/
│   ├── settings.json        # Configurações do workspace
│   └── ssh_config           # Configuração SSH para container
├── Cargo.toml               # Dependências: esp-hal, esp-println, esp-backtrace
├── build.rs                 # Build script para linkage
├── rust-toolchain.toml      # Toolchain riscv32imc-unknown-none-elf
├── Dockerfile               # Container Ubuntu 22.04 com Rust e SSH
├── podman-compose.yml       # Orquestração do container
├── tasks.md                 # Acompanhamento detalhado do progresso
├── prd.md                   # Especificações completas do produto
└── README.md                # Documentação geral
```

## Ambiente de Desenvolvimento

### Container Docker/Podman
- **Base**: Ubuntu 22.04
- **Usuário**: developer (senha: esp32dev)
- **SSH**: Porta 2222
- **Rust**: Stable com target riscv32imc-unknown-none-elf
- **Ferramentas**: espflash, esptool

### Comandos Importantes

```bash
# Iniciar container
podman-compose up -d

# Conectar via SSH
ssh developer@localhost -p 2222

# Compilar projeto
cargo build

# Flash no ESP32-C3
cargo run

# Parar container
podman-compose down
```

## Arquitetura Modular Planejada

### Módulos Implementados
- `config.rs` - Configurações centralizadas do sistema
- `wifi.rs` - Gestão de conexão WiFi com reconexão automática

### Módulos Futuros
- `counter.rs` - Contador 8-bit com overflow automático
- `mqtt.rs` - Cliente MQTT para comunicação
- `led_bar.rs` - Controle de 8 LEDs para visualização binária

### Fluxo de Desenvolvimento
1. **Fase 1**: ✅ Configuração inicial e blink
2. **Fase 2**: 🔄 Contador 8-bit com LEDs
3. **Fase 3**: ✅ Conectividade WiFi
4. **Fase 4**: 🔄 Cliente MQTT
5. **Fase 5**: 🔄 Controle remoto via MQTT
6. **Fase 6**: 🔄 Otimizações e melhorias

## Configurações Específicas

### Cargo.toml
```toml
[dependencies]
esp-hal = { version = "0.23.1", features = ["esp32c3"] }
esp-println = { version = "0.13.0", features = ["esp32c3"] }
esp-backtrace = { version = "0.15.0", features = ["esp32c3", "panic-handler", "exception-handler", "println"] }
esp-wifi = { version = "0.11.0", features = ["esp32c3"] }
smoltcp = { version = "0.11.0", default-features = false, features = ["medium-ethernet", "proto-dhcpv4", "proto-ipv4", "proto-ipv6", "socket-dhcpv4", "socket-icmp", "socket-raw", "socket-tcp", "socket-udp"] }
```

### Target e Toolchain
- **Target**: riscv32imc-unknown-none-elf
- **Runner**: espflash flash --monitor
- **Build-std**: core

## Comandos de Desenvolvimento

### Compilação e Debug
```bash
# Compilar
cargo build

# Compilar com otimizações
cargo build --release

# Compilar release + gerar .bin
./build-bin.sh

# Compilar debug + gerar .bin
./build-bin.sh debug

# Verificar código
cargo check

# Flash com monitor
cargo run

# Apenas flash
espflash flash target/riscv32imc-unknown-none-elf/debug/esp32-c3-mqtt-counter
```

### Testes
```bash
# Testes unitários (quando implementados)
cargo test

# Verificar sintaxe
cargo clippy

# Formatação
cargo fmt
```

## Configuração MQTT (Futura)

### Broker
- **Host**: broker.local (configurável)
- **Porta**: 1883
- **Cliente**: esp32-counter

### Tópicos
- `esp32/counter/value` - Valor atual do contador
- `esp32/counter/status` - Status do sistema
- `esp32/counter/cmd` - Comandos remotos

## Hardware

### ESP32-C3
- **Microcontrolador**: RISC-V 32-bit, 160MHz
- **Memória**: 400KB SRAM, 4MB Flash
- **WiFi**: 802.11 b/g/n
- **GPIO**: GPIO8 para LED onboard

### Pinout Planejado
- **GPIO8**: LED onboard (atual)
- **GPIO0-7**: Barra de 8 LEDs (futuro)
- **GPIO18,19**: UART (debug)

## Troubleshooting

### Problemas Comuns
1. **Device not found**: Verificar conexão USB e permissões
2. **Compilation errors**: Verificar toolchain instalado
3. **SSH connection refused**: Verificar se container está rodando

### Logs
```bash
# Logs do container
podman-compose logs esp32-dev

# Monitor serial
espflash monitor /dev/ttyUSB0
```

## Próximos Passos

### Imediatos
1. Testar compilação do exemplo blink
2. Verificar flash no ESP32-C3
3. Implementar contador 8-bit

### Médio Prazo
1. Implementar barra de LEDs
2. Adicionar conectividade WiFi
3. Integrar cliente MQTT

## Comandos para Claude

### Compilação
```bash
# Para testar build
cargo build

# Para gerar .bin file
./build-bin.sh

# Para flash no dispositivo
cargo run
```

### Estrutura de Arquivos
- Manter modularidade
- Documentar mudanças em tasks.md
- Atualizar prd.md conforme necessário

---

*Última atualização: 2025-07-15*  
*Status: Configuração inicial completa, pronto para Fase 2*