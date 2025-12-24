#[derive(Default)]
pub struct Param<T> {
    value: T,
    min: T,
    max: T,
    step: T,
    base: T,
}

impl<T> Param<T> {
    pub fn min(mut self, min: T) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: T) -> Self {
        self.max = max;
        self
    }

    pub fn step(mut self, step: T) -> Self {
        self.step = step;
        self
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}

impl<T> Param<T>
where
    T: Copy,
{
    pub fn base(mut self, base: T) -> Self {
        self.base = base;
        self.value = base;
        self
    }

    pub fn reset(&mut self) {
        self.value = self.base;
    }
}

impl<T> Param<T>
where
    T: Copy + std::cmp::Ord + std::ops::Add<T, Output = T> + std::ops::Sub<T, Output = T>,
{
    pub fn inc(&mut self) {
        if self.value <= self.max - self.step {
            self.value = self.value + self.step;
        } else {
            self.value = self.max;
        }
    }

    pub fn dec(&mut self) {
        if self.value >= self.min + self.step {
            self.value = self.value - self.step;
        } else {
            self.value = self.min;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_param() {
        let param = Param::<usize>::default().max(10).step(2).base(4);

        assert_eq!(param.min, usize::default());
        assert_eq!(param.max, 10);
        assert_eq!(param.base, 4);
        assert_eq!(param.step, 2);
        assert_eq!(*param.value(), 4);
    }

    #[test]
    fn change_value() {
        let mut param = Param::<usize>::default().min(1).max(10).step(2).base(4);

        assert_eq!(*param.value(), 4);
        param.inc();
        assert_eq!(*param.value(), 6);
        param.dec();
        assert_eq!(*param.value(), 4);
    }

    #[test]
    fn change_value_near_bounds() {
        let mut param = Param::<usize>::default().min(1).max(10).step(5).base(4);

        assert_eq!(*param.value(), 4);
        param.dec();
        assert_eq!(*param.value(), 1);
        param.inc();
        assert_eq!(*param.value(), 6);
        param.inc();
        assert_eq!(*param.value(), 10);
    }
}
