use std::cmp;
use std::fs;

#[derive(Default)]
pub struct Row {
    string: String
}

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        self.string.get(start..end).unwrap_or_default().to_string()
    }
}

impl Document {
    pub fn open(path: &str) -> Result<Self, std::io::Error> {
        let mut contents = fs::read_to_string(path)?;
        let mut rows = Vec::new();

        for value in contents.lines(){
            rows.push(Row {string: value.to_string()});
        }
        Ok(Self {rows: rows})
    }
    pub fn get_row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}