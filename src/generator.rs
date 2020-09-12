pub trait Generator {
    type Item;
    type Iterator: Iterator<Item = Self::Item> + Send;

    fn iter(&self) -> Self::Iterator;
}
