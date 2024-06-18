#!/bin/bash

# Enable X11 Forwarding
xhost + > /dev/null

# Source: https://medium.com/@paliwalsamriddhi/gui-apps-within-a-docker-container-971681838fda
docker run --rm -it \
    --name reenix-dev \
    -e DISPLAY=$DISPLAY \
    -v /tmp/.X11-unix:/tmp/.X11-unix \
    -v .:/reenix \
    reenix:latest