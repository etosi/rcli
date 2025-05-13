use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit_no: u8,
}

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut ret = Vec::with_capacity(128);

    let mut reader = csv::Reader::from_path(input)?;
    for result in reader.deserialize() {
        let p: Player = result?;
        ret.push(p);
    }

    let json = serde_json::to_string(&ret)?;
    fs::write(output, json)?;

    Ok(())
}
