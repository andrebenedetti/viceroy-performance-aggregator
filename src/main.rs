pub mod log_reader;
pub mod measurements;

fn main() {
    let (mut ram_usage, mut run_time_us) = log_reader::read_stdin();

    run_time_us.sort_unstable();
    println!(
        "{}",
        measurements::aggregate_sorted_measurements(&run_time_us)
    );
}
