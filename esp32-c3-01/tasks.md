# Tasks - ESP32-C3 MQTT Counter Project

## Fase 1: Configuração Inicial ✅

### 1.1 Ambiente de Desenvolvimento ✅
- [x] Criar estrutura inicial do projeto
- [x] Configurar Dockerfile para desenvolvimento via SSH
- [x] Configurar podman-compose.yml
- [x] Criar arquivos de configuração Rust (.cargo/config.toml, rust-toolchain.toml)
- [x] Implementar exemplo blink básico
- [x] Testar compilação do exemplo blink

## Fase 2: Contador 8-bit 🔄

### 2.1 Implementação do Contador
- [ ] Criar módulo counter.rs
- [ ] Implementar estrutura Counter com 8 bits
- [ ] Adicionar incremento automático a cada 1 segundo
- [ ] Integrar contador ao main.rs
- [ ] Testar overflow do contador (255 -> 0)

### 2.2 Barra de LEDs
- [ ] Definir pinos GPIO para 8 LEDs
- [ ] Implementar módulo led_bar.rs
- [ ] Criar função para exibir valor do contador em binário
- [ ] Integrar barra de LEDs ao contador
- [ ] Testar visualização dos valores

## Fase 3: Conectividade WiFi 🔄

### 3.1 Configuração WiFi
- [ ] Adicionar dependência esp-wifi ao Cargo.toml
- [ ] Criar módulo wifi.rs
- [ ] Implementar conexão ao access point
- [ ] Adicionar tratamento de reconexão
- [ ] Testar estabilidade da conexão

### 3.2 Configuração de Rede
- [ ] Implementar configuração estática de IP (opcional)
- [ ] Adicionar monitoramento de status da conexão
- [ ] Implementar indicadores visuais de conectividade

## Fase 4: Cliente MQTT 🔄

### 4.1 Implementação MQTT
- [ ] Adicionar dependência para cliente MQTT
- [ ] Criar módulo mqtt.rs
- [ ] Implementar conexão ao broker Mosquitto
- [ ] Configurar tópicos de publish/subscribe
- [ ] Testar envio de mensagens

### 4.2 Integração do Contador
- [ ] Enviar valor do contador via MQTT a cada segundo
- [ ] Implementar formato de mensagem JSON
- [ ] Adicionar timestamp às mensagens
- [ ] Implementar tratamento de erros MQTT

## Fase 5: Funcionalidades Avançadas 🔄

### 5.1 Controle Remoto
- [ ] Implementar recepção de comandos via MQTT
- [ ] Adicionar comando para reset do contador
- [ ] Implementar comando para alterar intervalo
- [ ] Adicionar comando para pause/resume

### 5.2 Monitoramento
- [ ] Implementar envio de status do sistema
- [ ] Adicionar métricas de uptime
- [ ] Implementar heartbeat
- [ ] Adicionar logs de debug via MQTT

## Fase 6: Otimizações e Melhorias 🔄

### 6.1 Gestão de Energia
- [ ] Implementar modos de baixo consumo
- [ ] Otimizar intervalos de transmissão
- [ ] Adicionar watchdog timer

### 6.2 Robustez
- [ ] Implementar recuperação de falhas
- [ ] Adicionar validação de configurações
- [ ] Implementar backup de estado
- [ ] Testes de stress

## Legenda
- ✅ Fase Completa
- 🔄 Fase em Desenvolvimento
- ⏳ Fase Planejada
- ❌ Fase Bloqueada

## Notas
- Cada fase deve ser completamente testada antes de avançar
- Manter documentação atualizada durante o desenvolvimento
- Implementar testes unitários quando aplicável
- Manter modularidade e separação de responsabilidades

---
*Última atualização: 2025-07-15*