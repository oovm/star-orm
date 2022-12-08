use std::marker::PhantomData;

pub use itertools::Itertools;

pub struct QueryPlanner<T> {
    pub batch: usize,
    pub query: String,
    pub item: PhantomData<T>,
}

impl<T> QueryPlanner<T> {
    pub fn new(batch: usize) -> Self {
        Self { batch, query: String::new(), item: Default::default() }
    }
    pub fn select(&mut self, columns: Vec<String>) -> &mut Self {
        self.query.push_str("select ");
        self.query.push_str(&columns.join(", "));
        self
    }
    pub fn from(&mut self, table: String) -> &mut Self {
        self.query.push_str(" from ");
        self.query.push_str(&table);
        self
    }
    pub fn r#where(mut self, predicate: fn(T) -> bool) -> Self {
        let _ = predicate;
        self
    }
    pub fn order_by<K, F>(mut self, key_selector: F) -> Self
    where
        K: Ord,
        F: FnMut(&T) -> K,
    {
        let _ = key_selector;
        self
    }
    pub fn group_by(&mut self, key_selector: String) -> &mut Self {
        self.query.push_str(" group by ");
        self.query.push_str(&key_selector);
        self
    }
    pub fn limit(&mut self, count: usize) -> &mut Self {
        self.query.push_str(" limit ");
        self.query.push_str(&count.to_string());
        self
    }
    pub fn build(&self) -> String {
        self.query.clone()
    }
}
