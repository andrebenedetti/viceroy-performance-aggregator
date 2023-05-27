# Viceroy Performance Aggregator

Read and aggregate request run time and RAM usage outputted from code running on Viceroy.

Designed to work with Fastly's Compute@Edge service.

# Usage

Pipe the output of your Compute@Edge service into the viceroy-performance-aggregator. 
To properly pipe the output, you need to run the Compute@Edge service with `fastly compute serve --skip-build`

## Example

1. Clone this repository
2. `cd` into your Compute@Edge repo
3. run `fastly compute serve --skip-build | cargo run --manifest-path path-to-viceroy-performance-aggregator/Cargo.toml` 
