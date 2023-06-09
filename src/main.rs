pub mod log_reader;
pub mod measurements;

fn main() {
    let (mut ram_usage, mut run_time_us) = log_reader::capture_measurements();

    run_time_us.sort_unstable();
    ram_usage.sort_unstable();
    println!(
        "{}",
        measurements::aggregate_sorted_measurements(&run_time_us)
    );
    println!(
        "{}",
        measurements::aggregate_sorted_measurements(&ram_usage)
    );
}
