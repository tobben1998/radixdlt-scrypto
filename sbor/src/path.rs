use crate::any::Value;
use crate::rust::vec::Vec;
use crate::rust::vec;
use sbor::*;

#[derive(Eq, PartialEq, Clone)]
pub struct MutableSborPath(Vec<usize>);

impl MutableSborPath {
    pub fn new() -> Self {
        MutableSborPath(vec![])
    }

    pub fn push(&mut self, path: usize) {
        self.0.push(path);
    }

    pub fn pop(&mut self) {
        self.0.pop();
    }
}

impl From<MutableSborPath> for SborPath {
    fn from(mutable: MutableSborPath) -> Self {
        SborPath::new(mutable.0)
    }
}

/// A series of indexes which describes some value in the sbor tree
#[derive(Eq, PartialEq, Clone)]
pub struct SborPath(Vec<usize>);

impl SborPath {
    pub fn new(path: Vec<usize>) -> Self {
        SborPath(path)
    }

    pub fn get_from_value<'a>(&'a self, value: &'a Value) -> Option<&'a Value> {
        let rel_path = SborValueRetriever(&self.0);
        rel_path.get_from(value)
    }

    pub fn get_from_value_mut<'a>(&'a self, value: &'a mut Value) -> Option<&'a mut Value> {
        let rel_path = SborValueRetriever(&self.0);
        rel_path.get_from_mut(value)
    }
}

/// Helper structure which helps in retrieving a value given a root value and sbor path
struct SborValueRetriever<'a>(&'a [usize]);

impl<'a> SborValueRetriever<'a> {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn pop(&self) -> (usize, Self) {
        let (index_slice, extended_path) = self.0.split_at(1);
        let index = index_slice[0];
        (index, SborValueRetriever(extended_path))
    }

    fn get_from_vector(&self, values: &'a [Value]) -> Option<&'a Value> {
        let (index, next_path) = self.pop();
        values
            .get(index)
            .and_then(|value| next_path.get_from(value))
    }

    fn get_from(self, value: &'a Value) -> Option<&'a Value> {
        if self.is_empty() {
            return Option::Some(value);
        }

        match value {
            Value::Struct { fields } | Value::Enum { fields, .. } => self.get_from_vector(fields),
            Value::Array { elements, .. } | Value::Vec { elements, .. } => {
                self.get_from_vector(elements)
            }
            _ => Option::None,
        }
    }

    fn get_from_vector_mut(&self, values: &'a mut [Value]) -> Option<&'a mut Value> {
        let (index, next_path) = self.pop();
        values
            .get_mut(index)
            .and_then(|value| next_path.get_from_mut(value))
    }

    fn get_from_mut(self, value: &'a mut Value) -> Option<&'a mut Value> {
        if self.is_empty() {
            return Option::Some(value);
        }

        match value {
            Value::Struct { fields } | Value::Enum { fields, .. } => self.get_from_vector_mut(fields),
            Value::Array { elements, .. } | Value::Vec { elements, .. } => {
                self.get_from_vector_mut(elements)
            }
            _ => Option::None,
        }
    }
}
