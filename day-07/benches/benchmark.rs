fn main() {
    #[cfg(feature = "bench")]
    divan::main();

    #[cfg(feature = "bench")]
    day_07::part1::benchmarks::main();

    #[cfg(feature = "bench")]
    day_07::part2::benchmarks::main();
}
