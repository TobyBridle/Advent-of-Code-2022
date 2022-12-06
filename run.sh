#!/bin/bash

while [[ true ]]; do
    read -p "Which day should be run? (e.g 5) " day
    if [[ $day =~ ^[0-9]+$ ]]; then
        if [[ $day -gt 0 && $day -lt 26 ]]; then
            break
        fi
    fi
    echo "Invalid day, try again"
done

cd Day\ $day/Day-$day/src
cargo run --release --bin Day-$day
