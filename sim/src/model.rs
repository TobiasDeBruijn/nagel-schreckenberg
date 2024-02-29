use crate::transformers::Transformer;

pub struct Model<R>(R);

impl<R> Model<R> {
    pub fn new(r: R) -> Self {
        Self(r)
    }

    pub fn apply<T: Transformer<R>>(mut self, t: T) -> Self {
        self.0 = t.transform(self.0);
        self
    }

    pub fn finish(self) -> R {
        self.0
    }
}
