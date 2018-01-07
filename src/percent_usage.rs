pub trait PercentUsage {
    fn percent_usage(&mut self) -> f64; // return a number between 0.0 (0%) and 1.0 (100%)
}
