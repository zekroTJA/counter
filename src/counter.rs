use std::{collections::HashMap, error::Error};

#[derive(Debug)]
pub struct Counter {
    values: HashMap<String, i32>,
}

impl Counter {
    pub fn get(&self, key: &String) -> Option<&i32> {
        self.values.get(key)
    }

    pub fn set(&mut self, key: String, value: i32) {
        self.values.insert(key, value);
    }

    pub fn modify(&mut self, key: String, value: i32) -> i32 {
        let val = self.get(&key).unwrap_or(&0);
        let val = *val + value;
        self.set(key, val);
        val
    }
}

pub trait Parse {
    fn encode(&self) -> Vec<u8>;
    fn decode(data: &[u8]) -> Result<Counter, Box<dyn Error>>;
}

impl Parse for Counter {
    fn encode(&self) -> Vec<u8> {
        let string_list: Vec<String> = self
            .values
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect();
        string_list.join("\n").into_bytes()
    }

    fn decode(data: &[u8]) -> Result<Counter, Box<dyn Error>> {
        let data_string = String::from_utf8_lossy(data).to_string();
        let line_splits: Vec<Vec<&str>> = data_string
            .split("\n")
            .filter(|line| line.trim().len() > 0)
            .map(|line| line.split(":"))
            .filter(|split| split.clone().count() > 0)
            .map(|split| split.collect::<Vec<&str>>())
            .collect();

        let mut values: HashMap<String, i32> = HashMap::new();
        for ls in line_splits {
            let key = String::from(ls[0]);
            let val = match ls[1].trim().parse::<i32>() {
                Ok(v) => v,
                Err(err) => return Err(err.into()),
            };
            values.insert(key, val);
        }

        Ok(Counter { values })
    }
}
