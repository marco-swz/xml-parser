#!/usr/bin/env
docker build -t temp .
docker run --name temp temp cargo run release
docker cp temp:/app/out.xml .
