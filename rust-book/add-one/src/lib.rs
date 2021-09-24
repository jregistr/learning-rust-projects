use rand::{thread_rng, Rng};

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn add_rand(x: i32) -> i32 {
    let mut rng = thread_rng();
    let n: i32 = rng.gen_range(8..130);
    x + n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_one(3), 4);
    }
}
