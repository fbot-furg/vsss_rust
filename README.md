# fbot-vss-rust
Rust version

**Precisa instalar rust**

Comandos para instalar o FIRASim estão no arquivo `install-commands.md`

## Protos
https://github.com/robocin/ssl-client/tree/master/include/ssl-client/protobuf-files/pb/proto
https://github.com/tokio-rs/prost

## Executando Exemplos
Exemplos são executados no robô amarelo de id 0

- Goleiro: `cargo run --example goalie`
- Seguidor de Bola: `cargo run --example follow_ball`

# TODO

sudo apt-get install qtbase5-dev qtchooser qt5-qmake qtbase5-dev-tools

sudo apt-get install -y build-essential g++ cmake git libqt5opengl5-dev libgl1-mesa-dev libglu1-mesa-dev libprotobuf-dev protobuf-compiler libode-dev libboost-dev

# Installing Docker Engine
# https://docs.docker.com/engine/install/ubuntu/

sudo apt-get update

sudo apt-get install \
    ca-certificates \
    curl \
    gnupg \
    lsb-release

sudo mkdir -p /etc/apt/keyrings
 curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg

echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

sudo apt-get update
sudo apt-get install docker-ce docker-ce-cli containerd.io docker-compose-plugin

sudo service docker start
sudo docker run hello-world

# --------------------------------------------------


# Installing container

sudo sh dockerbuild.sh
sudo sh rundocker.sh
