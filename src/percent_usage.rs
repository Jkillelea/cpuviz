pub trait PercentUsage {
    /// The percent usage of a resource (eg: the CPU usage). Should return a value between 0.0 (0%) and 1.0 (100%)
    fn percent_usage(&mut self) -> f64; // return a number between 0.0 (0%) and 1.0 (100%)
}
