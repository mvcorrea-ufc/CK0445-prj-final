# VSCode Configuration for ESP32-C3 Development

## Configuração SSH

### Opção 1: Conexão Direta pelo VSCode
1. Instale a extensão "Remote - SSH"
2. Pressione `F1` e digite "Remote-SSH: Connect to Host"
3. Digite: `developer@localhost:2222`
4. Senha: `esp32dev`

### Opção 2: Configuração SSH Permanente
Adicione ao seu `~/.ssh/config`:

```
Host esp32-c3-dev
    HostName localhost
    Port 2222
    User developer
    StrictHostKeyChecking no
    UserKnownHostsFile /dev/null
    PasswordAuthentication yes
    PreferredAuthentications password
    LogLevel QUIET
```

Depois conecte usando: `ssh esp32-c3-dev`

## Extensões Recomendadas

As seguintes extensões serão sugeridas automaticamente:
- **rust-analyzer**: Análise de código Rust
- **vscode-lldb**: Debugging para Rust
- **crates**: Gerenciamento de dependências
- **even-better-toml**: Suporte para arquivos TOML
- **remote-ssh**: Conexão SSH (essencial)

## Comandos Disponíveis

### Via Command Palette (F1)
- **Tasks: Run Task** → `cargo build`
- **Tasks: Run Task** → `build-bin release` (release + .bin)
- **Tasks: Run Task** → `build-bin debug` (debug + .bin)
- **Tasks: Run Task** → `cargo run`
- **Tasks: Run Task** → `cargo check`
- **Tasks: Run Task** → `cargo clippy`
- **Tasks: Run Task** → `cargo fmt`
- **Tasks: Run Task** → `espflash monitor`

### Via Terminal
```bash
# Compilar
cargo build

# Compilar release + gerar .bin
./build-bin.sh

# Compilar debug + gerar .bin
./build-bin.sh debug

# Flash no ESP32-C3
cargo run

# Verificar código
cargo check

# Linting
cargo clippy

# Formatação
cargo fmt

# Monitor serial
espflash monitor /dev/ttyUSB0
```

## Configurações Específicas

### Rust Analyzer
- **Target**: `riscv32imc-unknown-none-elf`
- **Features**: `esp32c3`
- **Check command**: `clippy`

### Editor
- **Format on save**: Habilitado
- **Code actions on save**: Clippy fixes automáticos

### Terminal
- **Default profile**: bash
- **Working directory**: `/workspace`

## Troubleshooting

### Conexão SSH
- Verifique se o container está rodando: `podman-compose ps`
- Teste a conexão: `ssh developer@localhost -p 2222`
- Reinicie o container se necessário: `podman-compose restart`

### Rust Analyzer
- Se houver problemas de análise, recarregue a janela: `F1` → "Developer: Reload Window"
- Verifique se o target está correto: `rustup target list --installed`

### Debugging
- Para debug detalhado, use o terminal integrado
- Logs do ESP32-C3 aparecem no `cargo run`
- Use `espflash monitor` para monitoramento contínuo

---

*Configuração otimizada para desenvolvimento ESP32-C3 com esp-hal*