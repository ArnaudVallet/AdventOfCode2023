use super::boat::Boat;

#[derive(Debug)]
pub struct Race {
    pub time: u64, // Time for the run
    pub record: u64, // Best distance reached
    pub runs: Vec<Boat>
}

impl Race {
    pub fn evaluate_each_run(&mut self) -> &Self {
        let mut runs: Vec<Boat> = vec![];
        for btn_hold_time in 0..=self.time {
            runs.push(Boat{
                speed: btn_hold_time,
                distance: btn_hold_time * (self.time - btn_hold_time)
            });
        }
        self.runs = runs;
        self
    }

    pub fn find_better_runs(&self) -> Vec<Boat> {
        self.runs
            .iter()
            .cloned()
            .filter(|&run| run.distance > self.record)
            .collect()
    }
}