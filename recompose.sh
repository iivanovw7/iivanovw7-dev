#!/bin/bash
imageName=iivanovw7-dev
containerName=iivanovw7-dev

docker build -t $imageName -f Dockerfile .

echo Delete old container...
docker rm -f $containerName

echo Run new container...
docker run -d -p 3003:8080 --name $containerName $imageName
