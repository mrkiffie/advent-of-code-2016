run day part:
    cargo run --package {{day}} --bin part-{{part}}

dhat day part:
    cargo run --profile dhat --features dhat-heap --package {{day}} --bin part-{{part}}
