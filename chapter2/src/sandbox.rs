#[derive(Debug)]
pub struct SomeNum {
    val: i32
}

impl SomeNum {
    pub fn new(val: i32) -> Self {
        SomeNum { val }
    }
}

trait AddOtherNum {
    fn add(&self, num: SomeNum) -> Self;
}

impl AddOtherNum for SomeNum {
    fn add(&self, num: SomeNum) -> SomeNum {
        SomeNum { val: self.val + num.val }
    }
}

pub fn add_some_nums(num_1: SomeNum, num_2: SomeNum) -> SomeNum {
    num_1.add(num_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let num_1 = SomeNum { val: 10 };
        let num_2 = SomeNum { val: 20 };

        let result = add_some_nums(num_1, num_2);

        assert_eq!(result.val, 30);
    }
}
