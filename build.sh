#!/bin/bash

docker build -t web-app .
docker run -i -p 8000:8000 --rm --name web web-app 
