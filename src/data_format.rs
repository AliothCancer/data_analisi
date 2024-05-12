use std::collections::HashMap;

impl Dataset {
    pub fn new<I, T>(columns: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: AsRef<[u8]>,
    {
        Dataset(HashMap::new())
    }
}

#[derive(Debug, Clone)]
pub struct Value {
    //column_id: usize,
    pub column_name: String,
    pub value: String,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(\n\tcol_name: {}\n\tvalue: {}\n)",
            self.column_name, self.value
        )
    }
}

pub struct Dataset(HashMap<String, Vec<Value>>);

pub struct Patient {
    sex: String,
    id: String,
    age: String,
    weight: String,
    simptoms: String,
    ecg: String,
    eco_basale: String,
    eco_stress: String,
    risk: String,
}
