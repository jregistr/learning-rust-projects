#[cfg(test)]
mod tests {
    use std::option::Option::Some;

    #[test]
    fn iter() {
        let v = vec![1, 2, 3, 4];
        let viter = v.iter();
        //references to the values inside v
        // we didn't make viter mutable because the for loop
        //takes ownership of viter and makes it mutable in background
        for val in viter {
            println!("{}", val);
        }

        let mut viter = v.iter();
        println!("{:?}", viter.next());
    }

    #[test]
    fn iter_while() {
        let v = vec![1, 2, 3, 4, 5, 7, 6];
        let mut viter = v.iter();
        println!("----- While Let ----");
        while let Some(num) = viter.next() {
            println!("{}", num);
        }
    }

    #[test]
    fn iter_take_ownership() {
        let v = vec![1, 5, 3, 7, 2, 4];
        // here, owned_iter took ownership of v
        let mut owned_iter = v.into_iter();
        println!("{:?}", owned_iter.next());
    }

    #[test]
    fn consuming_adapter_sum() {
        let v = vec![10, 10, 10];
        let iter = v.iter();
        // sum here takes ownership of iter
        let sum: i32 = iter.sum();
        println!("{}", sum);
        assert_eq!(30, sum);
    }

    // iterator adapters create another iterator
    // iterators are lazy. So creating an adapter actually does nothing until you use a consumer method
    #[test]
    fn adapter_then_consume() {
        let v = vec![0, 4, 9, 14];
        let iter = v.iter();

        let inc_sum = iter.map(|v| v + 1).sum::<i32>();
        println!("Increased Sum: {}", inc_sum);
        assert_eq!(31, inc_sum);
    }

    #[test]
    fn counter_iterator() {
        struct Counter {
            max: u32,
            count: u32,
            by: u32,
        }

        impl Iterator for Counter {
            type Item = u32;

            fn next(&mut self) -> Option<Self::Item> {
                let cur = self.count;
                if cur < self.max {
                    self.count = cur + self.by;
                    Some(cur)
                } else {
                    None
                }
            }
        }

        let mut c = Counter { max: 10, by: 2, count: 0 };
        while let Some(num) = c.next() {
            println!("{}", num);
        }

        let c = Counter { max: 10, by: 2, count: 0 };
        let d = Counter { max: 40, by: 2, count: 10 };
        let zipped_sum: u32 = c.zip(d.skip(2))
            .map(|(a, b)| a * b)
            .filter(|v| v % 2 == 0)
            .sum();
        println!("zipped: {}", zipped_sum);
        assert_eq!(400, zipped_sum);
    }
}
