#[derive(Debug, Eq, PartialEq, Hash)]
struct CustomEnum(i32, i32);

impl CustomEnum {
    fn new() -> Self {
        CustomEnum(0, 0)
    }
}

impl std::ops::Add for CustomEnum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        CustomEnum(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub for CustomEnum {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        CustomEnum(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Neg for CustomEnum {
    type Output = Self;

    fn neg(self) -> Self::Output {
        CustomEnum(-self.0, -self.1)
    }
}

fn main() {
    let mut hash1 = std::collections::HashMap::<CustomEnum, i32>::new();
    hash1.insert(CustomEnum(1,1), 2);
    let mut hash2 = std::collections::HashMap::<(i32, i32), i32>::new();
    hash2.insert((22,11), 33);
    println!("{:?}", hash1.get(&CustomEnum(1,1)));
    println!("{:?}", hash2);

    let sum = -CustomEnum(2,2);
    println!("{:?}", sum);
}
