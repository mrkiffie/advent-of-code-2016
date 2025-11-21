run day part:
    cargo run --package {{day}} --bin {{day}}--part{{part}}

dhat day part:
    cargo run --profile dhat --features dhat-heap --package {{day}} --bin {{day}}--part{{part}}

bench day:
    cargo bench --package {{day}} --features bench
