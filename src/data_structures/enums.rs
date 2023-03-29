#[derive(Debug)]
pub enum Fruit {
    Apple,
    Banana,
    Mango,
    Orange,
}

pub fn enums() -> Fruit {
    let fruit = Fruit::Apple;
    fruit
}