#![allow(unused)]

mod polymorph;
mod das_blog;
mod das_rust_blog;

fn main() {
    use avg::*;

    let mut avg_col = AverageCollection::new(vec![1, 2, 3, 4, 5]);
    println!("AVG: {}", avg_col.average()); // 3

    avg_col.add(10);
    println!("After Adding 10, AVG is: {:.2}", avg_col.average());

    println!("Popped: {:?}", avg_col.pop());
    println!("Now Avg is back to: {}", avg_col.average());
}

mod avg {
    pub struct AverageCollection {
        list: Vec<i32>,
        average: f64,
        sum: i32,
    }

    // associated functions
    impl AverageCollection {
        pub fn new(list: Vec<i32>) -> AverageCollection {
            let sum = AverageCollection::vec_sum(&list);
            let count = list.len();
            let average = AverageCollection::avg_calc(count, sum);
            AverageCollection { list, average, sum }
        }

        fn vec_sum(vec: &[i32]) -> i32 {
            vec.iter().sum()
        }

        fn avg_calc(count: usize, sum: i32) -> f64 {
            sum as f64 / count as f64
        }
    }

    // public methods
    impl AverageCollection {
        pub fn average(&self) -> f64 {
            self.average
        }

        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.sum += value;
            self.average = AverageCollection::avg_calc(self.list.len(), self.sum);
        }

        pub fn pop(&mut self) -> Option<i32> {
            let res = self.list.pop();
            if let Some(res) = res {
                self.sum -= res;
                self.average = AverageCollection::avg_calc(self.list.len(), self.sum);
                return Some(res);
            }
            None
        }
    }
}
