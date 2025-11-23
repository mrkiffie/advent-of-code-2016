fn main() {
    #[cfg(feature = "bench")]
    divan::main();

    #[cfg(feature = "bench")]
    day_04::part1::benchmarks::main();

    #[cfg(feature = "bench")]
    day_04::part2::benchmarks::main();
}
