# Dockerfile para desenvolvimento ESP32-C3 com Rust esp-hal
# Configurado para acesso via SSH e desenvolvimento com VSCode
FROM ubuntu:22.04

# Evitar prompts interativos durante a instalação
ENV DEBIAN_FRONTEND=noninteractive

# Instalar dependências do sistema incluindo SSH
RUN apt-get update && apt-get install -y \
    curl \
    git \
    build-essential \
    pkg-config \
    libudev-dev \
    libssl-dev \
    python3 \
    python3-pip \
    python3-venv \
    wget \
    unzip \
    openssh-server \
    sudo \
    vim \
    nano \
    && rm -rf /var/lib/apt/lists/*

# Configurar SSH
RUN mkdir /var/run/sshd && \
    echo 'root:esp32dev' | chpasswd && \
    sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config && \
    sed -i 's/#PasswordAuthentication yes/PasswordAuthentication yes/' /etc/ssh/sshd_config && \
    sed 's@session\s*required\s*pam_loginuid.so@session optional pam_loginuid.so@g' -i /etc/pam.d/sshd

# Criar usuário de desenvolvimento
RUN useradd -m -s /bin/bash -G sudo developer && \
    echo 'developer:esp32dev' | chpasswd && \
    echo 'developer ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

# Instalar Rust para o usuário developer
USER developer
WORKDIR /home/developer

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/home/developer/.cargo/bin:${PATH}"

# Configurar toolchain Rust para ESP32-C3
RUN rustup install stable && \
    rustup default stable && \
    rustup component add rust-src && \
    rustup target add riscv32imc-unknown-none-elf

# Instalar espflash para programação do ESP32
RUN cargo install espflash

# Voltar para root para instalar Python tools
USER root
RUN python3 -m pip install --upgrade pip && \
    python3 -m pip install esptool

# Criar diretório de trabalho
RUN mkdir -p /workspace && chown developer:developer /workspace
USER developer
WORKDIR /workspace

# Configurar variáveis de ambiente
ENV ESP_LOG=info
ENV RUST_LOG=info

# Expor porta SSH
EXPOSE 22

# Expor porta para possível servidor web futuro
EXPOSE 8080

# Comando para iniciar SSH daemon
USER root
CMD ["/usr/sbin/sshd", "-D"]