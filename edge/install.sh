echo "Downloading NATS 2.9.3"
curl -fLO https://github.com/nats-io/nats-server/releases/download/v2.9.3/nats-server-v2.9.3-linux-arm64.tar.gz
    
echo "Downloading wasmCloud host 0.57.4"
curl -fLO https://github.com/wasmCloud/wasmcloud-otp/releases/download/v0.57.4/aarch64-linux.tar.gz

echo "Extracting..."
mkdir wasmCloudHost_57-4
tar -xf aarch64-linux.tar.gz --directory ./wasmCloudHost_57-4
tar -xf nats-server-v2.9.3-linux-arm64.tar.gz
