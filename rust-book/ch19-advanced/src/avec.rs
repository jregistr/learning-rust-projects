macro_rules! avec {
    ($($x: expr),*) => {{
        let mut t = Vec::new();
        $(
            t.push($x);
        )*
        t
    }};
    ($to_repeat: expr; $times: expr) => {{
        let mut t = Vec::new();
        for _ in 0..$times {
            t.push(1);
        }
        t
    }};
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_can_create_vec_of1() {
        let exp = vec![1];
        let res = avec! {1};
        assert_eq!(exp, res);
    }

    #[test]
    fn test_can_create_vec_of3() {
        let exp = vec![1, 2, 3];
        let res = avec! {1, 2, 3};
        assert_eq!(exp, res);
    }

    #[test]
    fn test_can_make_repeat() {
        let exp = vec![1; 5];
        let res = avec![1; 5];
        assert_eq!(exp, res);
    }


}
