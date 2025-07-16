# ESP32-C3 MQTT Counter Project

Projeto de contador MQTT para ESP32-C3 usando esp-hal sem RTOS.

## Configuração do Ambiente

### Pré-requisitos
- Podman/Docker
- ESP32-C3 conectado via USB

### Inicialização do Container

```bash
# Construir e iniciar o container
podman-compose up -d

# Conectar via SSH (senha: esp32dev)
ssh developer@localhost -p 2222

# Ou conectar diretamente
podman-compose exec esp32-dev bash
```

### Configuração do VSCode

1. Instalar extensão "Remote - SSH"
2. Conectar em `ssh://developer@localhost:2222`
3. Instalar extensão Rust no container

### Compilação e Flash

```bash
# Compilar
cargo build

# Compilar release + gerar .bin
./build-bin.sh

# Compilar debug + gerar .bin
./build-bin.sh debug

# Flash no dispositivo
cargo run
```

## Estrutura do Projeto

```
esp32-c3-01/
├── src/
│   └── main.rs          # Código principal
├── .cargo/
│   └── config.toml      # Configurações Rust
├── Cargo.toml           # Dependências
├── Dockerfile           # Container de desenvolvimento
├── podman-compose.yml   # Orchestração
├── rust-toolchain.toml  # Toolchain Rust
└── README.md           # Este arquivo
```

## Etapas do Projeto

1. ✅ Configuração inicial e teste blink
2. 🔄 Implementação de contador 8-bit
3. 🔄 Conexão WiFi
4. 🔄 Cliente MQTT
5. 🔄 Barra de LEDs
6. 🔄 Funcionalidades adicionais