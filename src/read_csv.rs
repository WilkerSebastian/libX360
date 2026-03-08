use csv::Reader;
use std::io::Result;
use std::path::Path;

#[derive(Debug, serde::Deserialize)]
pub struct LineCsv {
    pub note: u8,
    pub time: u16,
}

pub fn read_csv(path: &Path) -> Result<Vec<LineCsv>> {

    let mut rdr = Reader::from_path(path)?;
    let mut res = Vec::new();

    for result in rdr.deserialize() {
        let registro: LineCsv = result?;
        res.push(registro);
    }

    Ok(res)

}