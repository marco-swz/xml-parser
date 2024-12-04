#!/usr/bin/env
docker build -t temp8 .
docker run --name temp8 temp8 cargo run --release --bin task8
docker cp temp8:/app/out.xml .
