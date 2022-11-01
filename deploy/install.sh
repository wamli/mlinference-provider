#!/bin/bash
echo "Downloading NATS 2.9.4"
curl -fLO https://github.com/nats-io/nats-server/releases/download/v2.9.4/nats-server-v2.9.4-linux-amd64.tar.gz
echo "Downloading wasmCloud host 0.58.2"
curl -fLO https://github.com/wasmCloud/wasmcloud-otp/releases/download/v0.58.2/x86_64-linux.tar.gz
echo "Extracting..."
tar -xf x86_64-linux.tar.gz
tar -xf nats-server-v2.9.4-linux-amd64.tar.gz
#sudo mv nats-server-v2.9.4-linux-amd64.tar.gz/nats-server /usr/local/bin/