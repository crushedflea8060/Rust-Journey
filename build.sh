#!/bin/bash

docker build -t web-app .
docker run -d --rm --name web web-app 
