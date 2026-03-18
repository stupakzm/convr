use crate::formats::Format;
use anyhow::{bail, Result};
use std::path::Path;

pub fn convert(input: &Path, src: &Format, output: &Path, target: &Format) -> Result<()> {
    let content = std::fs::read_to_string(input)?;

    // Parse input into a generic JSON Value (common intermediate)
    let value: serde_json::Value = match src {
        Format::Json => serde_json::from_str(&content)?,
        Format::Yaml => serde_yaml::from_str(&content)?,
        Format::Toml => {
            let toml_val: toml::Value = toml::from_str(&content)?;
            serde_json::to_value(toml_val)?
        }
        Format::Csv  => csv_to_json(&content)?,
        Format::Xml  => xml_to_json(&content)?,
        _ => bail!("Unsupported data source format: {:?}", src),
    };

    // Serialize to target format
    let out = match target {
        Format::Json => serde_json::to_string_pretty(&value)?,
        Format::Yaml => serde_yaml::to_string(&value)?,
        Format::Toml => {
            let toml_val: toml::Value = serde_json::from_value(value)?;
            toml::to_string_pretty(&toml_val)?
        }
        Format::Csv  => json_to_csv(&value)?,
        Format::Xml  => json_to_xml(&value)?,
        _ => bail!("Unsupported data target format: {:?}", src),
    };

    std::fs::write(output, out)?;
    Ok(())
}

fn csv_to_json(content: &str) -> Result<serde_json::Value> {
    let mut rdr = csv::Reader::from_reader(content.as_bytes());
    let headers: Vec<String> = rdr.headers()?.iter().map(|s| s.to_string()).collect();
    let mut rows = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let obj: serde_json::Map<String, serde_json::Value> = headers
            .iter()
            .zip(record.iter())
            .map(|(k, v)| (k.clone(), serde_json::Value::String(v.to_string())))
            .collect();
        rows.push(serde_json::Value::Object(obj));
    }
    Ok(serde_json::Value::Array(rows))
}

fn json_to_csv(value: &serde_json::Value) -> Result<String> {
    let rows = value.as_array().ok_or_else(|| anyhow::anyhow!("JSON must be an array for CSV output"))?;
    if rows.is_empty() {
        return Ok(String::new());
    }
    let headers: Vec<String> = rows[0]
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("Each JSON element must be an object"))?
        .keys()
        .cloned()
        .collect();

    let mut wtr = csv::Writer::from_writer(vec![]);
    wtr.write_record(&headers)?;
    for row in rows {
        let obj = row.as_object().ok_or_else(|| anyhow::anyhow!("Non-object in array"))?;
        let record: Vec<String> = headers
            .iter()
            .map(|h| obj.get(h).map(|v| value_to_csv_cell(v)).unwrap_or_default())
            .collect();
        wtr.write_record(&record)?;
    }
    Ok(String::from_utf8(wtr.into_inner()?)?)
}

fn value_to_csv_cell(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Null => String::new(),
        other => other.to_string(),
    }
}

fn xml_to_json(content: &str) -> Result<serde_json::Value> {
    // Basic XML → JSON: parse with quick-xml, build a JSON object
    use quick_xml::events::Event;
    use quick_xml::Reader;

    let mut reader = Reader::from_str(content);
    reader.config_mut().trim_text(true);

    let mut stack: Vec<(String, serde_json::Map<String, serde_json::Value>)> = Vec::new();
    let mut root: Option<serde_json::Value> = None;

    loop {
        match reader.read_event()? {
            Event::Start(e) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                stack.push((name, serde_json::Map::new()));
            }
            Event::End(_) => {
                if let Some((name, obj)) = stack.pop() {
                    let val = serde_json::Value::Object(obj);
                    if stack.is_empty() {
                        let mut wrapper = serde_json::Map::new();
                        wrapper.insert(name, val);
                        root = Some(serde_json::Value::Object(wrapper));
                    } else if let Some((_, parent)) = stack.last_mut() {
                        parent.insert(name, val);
                    }
                }
            }
            Event::Text(e) => {
                let text = e.unescape()?.to_string();
                if !text.trim().is_empty() {
                    if let Some((_, obj)) = stack.last_mut() {
                        obj.insert("#text".to_string(), serde_json::Value::String(text));
                    }
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }

    root.ok_or_else(|| anyhow::anyhow!("Failed to parse XML"))
}

fn json_to_xml(value: &serde_json::Value) -> Result<String> {
    let mut output = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    write_xml_value(&mut output, "root", value);
    Ok(output)
}

fn write_xml_value(out: &mut String, tag: &str, value: &serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            out.push_str(&format!("<{tag}>"));
            for (k, v) in map {
                write_xml_value(out, k, v);
            }
            out.push_str(&format!("</{tag}>"));
        }
        serde_json::Value::Array(arr) => {
            for item in arr {
                write_xml_value(out, tag, item);
            }
        }
        serde_json::Value::String(s) => {
            out.push_str(&format!("<{tag}>{s}</{tag}>"));
        }
        other => {
            out.push_str(&format!("<{tag}>{other}</{tag}>"));
        }
    }
}
