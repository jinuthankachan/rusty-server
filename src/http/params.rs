use std::collections::HashMap;

#[derive(Debug)]
pub struct Params<'bfr> {
    data: HashMap<&'bfr str, Value<'bfr>>,
}

#[derive(Debug)]
pub enum Value<'bfr> {
    Single(&'bfr str),
    Multiple(Vec<&'bfr str>),
}

impl<'bfr> Params<'bfr> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'bfr> From<&'bfr str> for Params<'bfr> {
    fn from(qs: &'bfr str) -> Self {
        let mut data = HashMap::new();
        for each in qs.split('&') {
            let mut key = each;
            let mut val = "";
            if let Some(i) = each.find('=') {
                key = &each[..i];
                val = &each[i + 1..];
            }
            data.entry(key)
                .and_modify(|existing| match existing {
                    Value::Single(old_val) => *existing = Value::Multiple(vec![old_val, val]),
                    Value::Multiple(vec) => vec.push(val),
                })
                .or_insert(Value::Single(val));
        }
        Self { data }
    }
}
