# wamli - wasmCloud machine learning inference

~~~
wamli is an inference server supporting multiple engines and targets written in Rust
~~~

## Supported engines and targets

   Hardware         |    ONNX           |  Tensorflow       |  Tensorflow Lite        |
  :---------------  | :---------------: | :---------------: | :---------------        |
  CPU (x86, ARM)    | ✅                | ✅                |  ✅ (feature *tflite*)  |
  TPU (edge TPU)    |                   |                   |  ✅ (feature *edgetpu*) |
  GPU               |                   |                   |                         |
  
GPU support is on the roadmap.

> **_NOTE:_**  Even though there are binaries for Windows as well, **Linux** is assumed as an OS.

## Getting started

### Tool installation

An example application including multiple different models is pre-compiled and pre-configured such that its different parts only have to be assembled. In order to make that work, however, there is the list of the following tools which are assumed to be installed on a Linux like OS:

* [`wasmcloud host`](https://wasmcloud.dev/overview/installation/manual-install/)
* [`wash`](https://wasmcloud.dev/overview/installation/)
* `docker`, including [Compose v2](https://docs.docker.com/compose/#installing-compose-v2)
* `jq`, the lightweight and flexible command-line JSON processor
* `make`
* `nc`, aka *netcat*
* [`bindle`](https://github.com/deislabs/bindle/tags), recommendation is __v0.7.1__

### Configuration

Open __*deploy/env*__ and modify according to your needs:
* set `BINDLE_HOME`, e.g. `export BINDLE_HOME=$HOME/dev/rust/bindle`
* set `WASMCLOUD_HOST_HOME`, e.g. `export WASMCLOUD_HOST_HOME=$HOME/dev/wasmcloud/wasmCloudHost_57-4`

### Launching the application

```bash
# go where run.sh is
cd deploy

# start the model store (bindle server)
./run.sh bindle-start

# upload a bunch of example scripts to the model store
# this has to be done only once
./run.sh bindle-create

# assemble the application's packages
./run.sh packages

# run some inference requests, see `examples/inference.md` for more examples
curl --silent -T ../images/imagenet/cat.jpg localhost:8078/squeezenetv117/matches | jq

# stop the system in a controlled way
./run.sh wipe

# explore all the run script's options
./run.sh
```

## Open Source Tech Stack

[<img src="readme_files/webassembly_logo.png" alt="webassembly" height=40>](https://webassembly.org/)
[<img src="readme_files/wasmcloud_logo.png" alt="wasmcloud" height=40>](https://wasmcloud.dev/)
[<img src="readme_files/tract-horizontal-blue.png" alt="tract" height=40>](https://github.com/sonos/tract)
[<img src="readme_files/tflite.png" alt="tract" height=40>](https://www.tensorflow.org/lite)
[**Bindle**](https://github.com/deislabs/bindle)
