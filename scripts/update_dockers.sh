#!/bin/bash
for c in c1 c2 c3 c4; do 
    docker exec $c rm -f ~/worker
    docker cp ~/Computaci-n-distribuida/pi_mediante_monte_carlo/target/debug/worker $c:/home
    echo "$c actualizado"
done
