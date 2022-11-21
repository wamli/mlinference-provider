Do some inference requests via the http endpoint:

```bash
# in case the host runs on the development machine, use one of the next lines
curl --silent -T ../images/imagenet/cat.jpg localhost:8078/mobilenetv27/matches | jq
curl --silent -T ../images/imagenet/cat.jpg localhost:8078/mobilenetv27/matches | jq
curl --silent -T ../images/mnist/4.png localhost:8078/mnistv1/mnist/matches | jq
curl --silent -T ../images/imagenet/cat_edgetpu.bmp localhost:8078/mobilenetv1tpu | jq
curl --silent -T ../images/imagenet/cat.jpg localhost:8078/mobilenetv1tpu/matches/rgb8 | jq

# in order to send requests to an edge device with address 192.168.178.97 use next line(s) 
curl --silent -T ../images/imagenet/cat.jpg 192.168.178.97:8078/mobilenetv27/matches | jq
curl --silent -T ../images/imagenet/cat.jpg 192.168.178.97:8078/squeezenetv117/matches | jq
curl --silent -T ../images/mnist/4.png 192.168.178.97:8078/mnistv1/mnist/matches | jq
curl --silent -T ../images/imagenet/cat.jpg 192.168.178.97:8078/mobilenetv1tpu/matches/rgb8 | jq
```