use rand::Rng;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn add_random(x: i32) -> i32 {
    x + rand::thread_rng().gen_range(1..100)
}

#[cfg(test)]
mod i_eat_balls /* just proving your test module doesn't need to be named tests */ {
    use super::*;

    #[test]
    #[should_panic(expected = "overflow")]
    fn test_add_one() {
        let (x, y, z) = (16, -50, i32::MAX);

        assert_eq!(17, add_one(x));
        assert_eq!(-49, add_one(y));
        assert_eq!(i32::MIN, add_one(z)); //this should integer overflow and panic
    }
}