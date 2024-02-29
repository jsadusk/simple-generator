pub trait Generator {
    type Item;
    type Iterator: Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::Iterator;
}
