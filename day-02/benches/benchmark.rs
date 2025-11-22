fn main() {
    #[cfg(feature = "bench")]
    divan::main();

    #[cfg(feature = "bench")]
    day_02::part1::benchmarks::main();

    #[cfg(feature = "bench")]
    day_02::part2::benchmarks::main();
}
