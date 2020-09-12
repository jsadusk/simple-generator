use crate::generator::*;

pub struct GeneratorFunc<SetupFunc> {
    setup: SetupFunc,
}

pub struct GeneratorFuncIterator<IterFunc> {
    iterfunc: IterFunc,
}

impl<Value, IterFunc: FnMut() -> Option<Value>, SetupFunc: Fn() -> IterFunc>
    GeneratorFunc<SetupFunc>
{
    pub fn new(setup: SetupFunc) -> Self {
        Self { setup }
    }
}

impl<Value, IterFunc: Send + FnMut() -> Option<Value>> GeneratorFuncIterator<IterFunc> {
    fn new(iterfunc: IterFunc) -> Self {
        Self { iterfunc }
    }
}

impl<Value, IterFunc: Send + FnMut() -> Option<Value>> std::iter::Iterator
    for GeneratorFuncIterator<IterFunc>
{
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        (self.iterfunc)()
    }
}

impl<Value, IterFunc: Send + FnMut() -> Option<Value>, SetupFunc: Fn() -> IterFunc> Generator
    for GeneratorFunc<SetupFunc>
{
    type Iterator = GeneratorFuncIterator<IterFunc>;
    type Item = Value;

    fn iter(&self) -> Self::Iterator {
        GeneratorFuncIterator::new((self.setup)())
    }
}
