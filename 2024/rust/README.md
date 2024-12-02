generate new day folder:
    cargo generate --path ./template --name {day}

test specific day & part:
    cargo test -p {day} {part}

run specific day & part:
    cargo run -p {day} --bin {part}