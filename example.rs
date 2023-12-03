pub struct AveragedCollection {
    list: Vec<i32>,
    average: Option<f64>,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average.unwrap()
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = Some(total as f64 / self.list.len() as f64);
    }
}

fn main() {
    let mut a = AveragedCollection {
        list: Vec::new(),
        average: None,
    };
    a.add(2);
    a.add(4);
    println!("list: {:?}", a.list);
    println!("mean: {:?}", a.average());
}
