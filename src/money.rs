

#[derive(Debug,PartialEq)]
struct Doller {
    pub amount: i32
}

#[derive(Debug,PartialEq)]
struct Franc {
    pub amount: i32
}

trait Money<T: Money<T>> {
    fn amount(&mut self) -> &i32;
    fn times(&mut self, multiplier: &'static i32) -> Self;
    fn eqauls(&mut self, money: &mut T) -> bool{
        return self.amount() == money.amount();
    }
}


impl Money<Doller> for Doller {
    fn amount(&mut self) -> &i32 {
        return &self.amount;
    }
    fn times(&mut self, multiplier: &'static i32) -> Doller{
        return Doller {amount: self.amount * multiplier}
    }
}

impl Money<Franc> for Franc {
    fn amount(&mut self) -> &i32 {
        return &self.amount;
    }
    fn times(&mut self, multiplier: &'static i32) -> Franc{
        return Franc {amount: self.amount * multiplier}
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
        assert!(doller.eqauls(&mut Doller {amount: 5}));
        assert!(!doller.eqauls(&mut Doller {amount: 6}));
    }

    #[test]
    fn test_franc_multiplication(){
        let mut franc = Franc {amount: 5};
        assert_eq!(Franc {amount: 10}, franc.times(&2));
        assert_eq!(Franc {amount: 15}, franc.times(&3));
    }
}
