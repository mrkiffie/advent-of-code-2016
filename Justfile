run day part:
    cargo run --package {{day}} --bin {{day}}--part{{part}}

test day part:
    cargo test --package {{day}} --lib part{{part}}

dhat day part:
    cargo run --profile dhat --features dhat-heap --package {{day}} --bin {{day}}--part{{part}}

bench day:
    cargo bench --package {{day}} --features bench

generate day:
	cp -R template day-{{day}}
	sed -i '' 's/xx/{{day}}/' day-{{day}}/Cargo.toml
	sed -i '' 's/xx/{{day}}/' day-{{day}}/benches/benchmark.rs
	sed -i '' 's/xx/{{day}}/' day-{{day}}/src/bin/part1.rs
	sed -i '' 's/xx/{{day}}/' day-{{day}}/src/bin/part2.rs
