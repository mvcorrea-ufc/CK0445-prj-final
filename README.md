# CK0445 - Projeto Final - Sistemas Distribuídos

## Projeto ESP32-C3 MQTT Counter

Sistema embarcado baseado em ESP32-C3 que implementa um contador de 8 bits com transmissão MQTT e visualização em barra de LEDs, utilizando apenas esp-hal sem RTOS.

### Estrutura do Projeto

```
CK0445-prj-final/
├── esp32-c3-01/                    # Projeto ESP32-C3
│   ├── src/                        # Código fonte Rust
│   ├── .cargo/                     # Configurações Rust
│   ├── .vscode/                    # Configurações VSCode
│   ├── Dockerfile                  # Container desenvolvimento
│   ├── podman-compose.yml          # Orquestração container
│   ├── CLAUDE.md                   # Contexto para Claude
│   ├── tasks.md                    # Acompanhamento do progresso
│   ├── prd.md                      # Especificações do produto
│   └── README.md                   # Documentação específica
├── prj_files/                      # Documentos do projeto
│   ├── 2025.1 Especificação do Trabalho - RabbitMQ-gRPC.docx.pdf
│   ├── Capitulo05.pdf
│   └── Definindo Protocolo.pdf
└── README.md                       # Este arquivo
```

### Tecnologias Utilizadas

- **Hardware**: ESP32-C3 (RISC-V)
- **Linguagem**: Rust
- **HAL**: esp-hal v0.23.1 (bare-metal, sem RTOS)
- **Comunicação**: WiFi + MQTT
- **Desenvolvimento**: Docker/Podman + VSCode SSH

### Funcionalidades

#### Implementadas ✅
- Ambiente de desenvolvimento containerizado
- Exemplo blink básico
- Configuração completa do projeto
- Documentação detalhada

#### Planejadas 🔄
- Contador 8-bit com overflow automático
- Barra de 8 LEDs para visualização binária
- Conectividade WiFi
- Cliente MQTT para broker Mosquitto
- Controle remoto via comandos MQTT

### Configuração Rápida

```bash
# Clone o repositório
git clone https://github.com/mvcorrea-ufc/CK0445-prj-final.git
cd CK0445-prj-final/esp32-c3-01

# Inicie o container
podman-compose up -d

# Conecte via SSH (VSCode)
# Host: localhost:2222
# User: developer
# Password: esp32dev

# Compile o projeto
cargo build

# Flash no ESP32-C3
cargo run
```

### Desenvolvimento

Para desenvolvimento detalhado, consulte:
- [`esp32-c3-01/README.md`](esp32-c3-01/README.md) - Documentação específica
- [`esp32-c3-01/tasks.md`](esp32-c3-01/tasks.md) - Plano de desenvolvimento
- [`esp32-c3-01/prd.md`](esp32-c3-01/prd.md) - Especificações completas
- [`esp32-c3-01/CLAUDE.md`](esp32-c3-01/CLAUDE.md) - Contexto para Claude

### Fases do Projeto

1. **Fase 1** ✅ - Configuração inicial e blink
2. **Fase 2** 🔄 - Contador 8-bit com LEDs
3. **Fase 3** 🔄 - Conectividade WiFi
4. **Fase 4** 🔄 - Cliente MQTT
5. **Fase 5** 🔄 - Controle remoto via MQTT
6. **Fase 6** 🔄 - Otimizações e melhorias

### Autor

Projeto desenvolvido para a disciplina CK0445 - Sistemas Distribuídos  
Universidade Federal do Ceará (UFC)

### Licença

MIT OR Apache-2.0