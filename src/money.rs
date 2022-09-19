#[derive(Debug,PartialEq)]
struct Doller {
    pub amount: i32
}

#[derive(Debug,PartialEq)]
struct Franc {
    pub amount: i32
}

trait Money<T> {
    fn times(&mut self, multiplier: &'static i32) -> Self;
    fn eqauls(&mut self, money: &T) -> bool;
}


impl Money<Doller> for Doller {
    fn times(&mut self, multiplier: &'static i32) -> Doller{
        return Doller {amount: self.amount * multiplier}
    }
    fn eqauls(&mut self, money: &Doller) -> bool {
        return money.amount == self.amount;
    }
}

impl Money<Franc> for Franc {
    fn times(&mut self, multiplier: &'static i32) -> Franc{
        return Franc {amount: self.amount * multiplier}
    }
    fn eqauls(&mut self, money: &Franc) -> bool {
        return money.amount == self.amount;
    }
}

#[cfg(test)]
mod test{
    use super::Money;
    use super::Doller;
    use super::Franc;

    #[test]
    fn test_multiplication(){
        let mut doller = Doller {amount: 5};
        assert_eq!(Doller {amount: 10}, doller.times(&2));
        assert_eq!(Doller {amount: 15}, doller.times(&3));
    }

    #[test]
    fn test_equals(){
        let mut doller = Doller {amount: 5};
        assert!(doller.eqauls(&Doller {amount: 5}));
        assert!(!doller.eqauls(&Doller {amount: 6}));
    }

    #[test]
    fn test_franc_multiplication(){
        let mut franc = Franc {amount: 5};
        assert_eq!(Franc {amount: 10}, franc.times(&2));
        assert_eq!(Franc {amount: 15}, franc.times(&3));
    }
}
