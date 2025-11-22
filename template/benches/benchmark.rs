fn main() {
    #[cfg(feature = "bench")]
    divan::main();

    #[cfg(feature = "bench")]
    day_xx::part1::benchmarks::main();

    #[cfg(feature = "bench")]
    day_xx::part2::benchmarks::main();
}
