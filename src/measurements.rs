use core::fmt;

pub struct Measurements {
    avg: i32,
    p95: i32,
    p99: i32,
}

pub fn aggregate_sorted_measurements(measurements: &Vec<i32>) -> Measurements {
    let mut sum = 0;
    let len = measurements.len();
    for m in measurements {
        sum += m;
    }

    Measurements {
        avg: sum / len as i32,
        p95: measurements[(0.95 * measurements.len() as f32) as usize],
        p99: measurements[(0.99 * measurements.len() as f32) as usize],
    }
}

impl fmt::Display for Measurements {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "avg: {}\np95: {}\np99: {}", self.avg, self.p95, self.p99)
    }
}
