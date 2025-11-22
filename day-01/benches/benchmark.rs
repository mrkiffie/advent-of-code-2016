fn main() {
    #[cfg(feature = "bench")]
    divan::main();

    #[cfg(feature = "bench")]
    day_01::part1::benchmarks::main();

    #[cfg(feature = "bench")]
    day_01::part2::benchmarks::main();
}
