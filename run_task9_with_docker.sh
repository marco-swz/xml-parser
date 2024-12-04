#!/usr/bin/env
docker build -t temp9 .
docker run --name temp9 temp9 cargo run --release --bin task9
