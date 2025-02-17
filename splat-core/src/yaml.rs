use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::Result;
use serde::{Deserialize, Deserializer};
use serde_yaml::Value;

#[derive(Debug)]
pub enum YamlSegmentArgs {
    List(Vec<Value>),
    Dict(HashMap<String, Value>),
}

#[derive(Debug, Deserialize)]
pub struct SplatYaml {
    pub name: String,
    pub sha1: String,
    pub options: HashMap<String, Value>,
    pub vram_classes: Option<Vec<VramClass>>,
    pub segments: Vec<YamlSegment>,
}

#[derive(Debug, Deserialize)]
pub struct DictSegment {
    #[serde(rename = "type")]
    pub segment_type: String,
    pub start: Option<u64>,
    pub name: Option<String>,
    pub dir: Option<PathBuf>,
    pub vram: Option<u64>,
    pub subsegments: Option<Vec<DictSegment>>,
    pub bss_size: Option<u64>,
    pub vram_class: Option<String>,
    pub follows_vram: Option<String>,
    pub align: Option<u64>,
    pub subalign: Option<u64>,
    pub size: Option<u64>,
    pub symbol_name_format: Option<String>,
    pub args: Option<HashMap<String, Value>>,
}

impl From<DictSegment> for YamlSegment {
    fn from(ds: DictSegment) -> Self {
        YamlSegment {
            segment_type: ds.segment_type,
            rom: ds.start,
            name: ds.name,
            dir: ds.dir,
            vram: ds.vram,
            bss_size: ds.bss_size,
            subsegments: ds
                .subsegments
                .map(|ss| ss.into_iter().map(|s| s.into()).collect()),
            vram_class: ds.vram_class,
            follows_vram: ds.follows_vram,
            align: ds.align,
            subalign: ds.subalign,
            size: ds.size,
            symbol_name_format: ds.symbol_name_format,
            args: ds.args.map(|a| YamlSegmentArgs::Dict(a)),
        }
    }
}

#[derive(Debug)]
pub struct YamlSegment {
    pub segment_type: String,
    pub rom: Option<u64>,
    pub name: Option<String>,
    pub dir: Option<PathBuf>,
    pub vram: Option<u64>,
    pub subsegments: Option<Vec<YamlSegment>>,
    pub bss_size: Option<u64>,
    pub vram_class: Option<String>,
    pub follows_vram: Option<String>,
    pub align: Option<u64>,
    pub subalign: Option<u64>,
    pub size: Option<u64>,
    pub symbol_name_format: Option<String>,
    pub args: Option<YamlSegmentArgs>,
}

impl<'de> Deserialize<'de> for YamlSegment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize_segment(deserializer)
    }
}

fn parse_address(value: &Value) -> Result<Option<u64>, serde::de::value::Error> {
    match value {
        Value::Number(n) => {
            if let Some(num) = n.as_u64() {
                Ok(Some(num))
            } else if let Some(num) = n.as_i64() {
                Ok(Some(num as u64))
            } else {
                Err(serde::de::Error::custom(format!(
                    "Invalid numerical address value: {:}",
                    n
                )))
            }
        }
        Value::String(s) if s.to_lowercase() == "auto" => Ok(None),
        s => Err(serde::de::Error::custom(format!(
            "Invalid address value '{s:?}'."
        ))),
    }
}

fn parse_number(value: &Value) -> Result<Option<u64>, serde::de::value::Error> {
    match value {
        Value::Number(n) => {
            if let Some(num) = n.as_u64() {
                Ok(Some(num))
            } else if let Some(num) = n.as_i64() {
                Ok(Some(num as u64))
            } else {
                Err(serde::de::Error::custom(format!(
                    "Invalid numerical address value: {:}",
                    n
                )))
            }
        }
        s => Err(serde::de::Error::custom(format!(
            "Invalid address value '{s:?}'."
        ))),
    }
}

fn deserialize_segment<'de, D>(deserializer: D) -> Result<YamlSegment, D::Error>
where
    D: Deserializer<'de>,
{
    match Value::deserialize(deserializer)? {
        Value::Mapping(mapping) => {
            // parse the mapping into a YamlSegment
            let mut segment_type = None;
            let mut rom = None;
            let mut name = None;
            let mut dir = None;
            let mut vram = None;
            let mut align = None;
            let mut subalign = None;
            let mut bss_size = None;
            let mut subsegments = None;
            let mut vram_class = None;
            let mut follows_vram = None;
            let mut size = None;
            let mut symbol_name_format = None;
            let mut args: Option<HashMap<String, Value>> = None;

            for (key, value) in mapping {
                match key.as_str() {
                    Some("type") => {
                        segment_type = match value {
                            Value::String(s) => Some(s),
                            _ => {
                                return Err(serde::de::Error::custom(format!(
                                    "Invalid type value {:?}.",
                                    value
                                )))
                            }
                        };
                    }
                    Some("start") => {
                        rom = parse_address(&value).map_err(serde::de::Error::custom)?;
                    }
                    Some("name") => {
                        name = match value {
                            Value::String(s) => Some(s),
                            _ => return Err(serde::de::Error::custom("Invalid name value.")),
                        };
                    }
                    Some("dir") => {
                        dir = match value {
                            Value::String(s) => Some(PathBuf::from(s)),
                            _ => return Err(serde::de::Error::custom("Invalid dir value.")),
                        };
                    }
                    Some("vram") => {
                        vram = match value {
                            Value::Number(n) => {
                                if let Some(num) = n.as_u64() {
                                    Some(num)
                                } else if let Some(num) = n.as_i64() {
                                    Some(num as u64) //Safe because of prev check.
                                } else {
                                    return Err(serde::de::Error::custom(format!(
                                        "Invalid numerical vram value: {:}",
                                        n
                                    )));
                                }
                            }
                            _ => return Err(serde::de::Error::custom("Invalid vram value.")),
                        };
                    }
                    Some("subsegments") => {
                        subsegments = match value {
                            Value::Sequence(l) => {
                                let mut subsegments = Vec::new();
                                for v in l {
                                    subsegments.push(
                                        deserialize_segment(v).map_err(serde::de::Error::custom)?,
                                    );
                                }
                                Some(subsegments)
                            }
                            _ => {
                                return Err(serde::de::Error::custom("Invalid subsegments value."))
                            }
                        };
                    }
                    Some("bss_size") => {
                        bss_size = parse_number(&value).map_err(serde::de::Error::custom)?;
                    }
                    Some("vram_class") => {
                        vram_class = match value {
                            Value::String(s) => Some(s),
                            _ => return Err(serde::de::Error::custom("Invalid vram_class value.")),
                        };
                    }
                    Some("follows_vram") => {
                        follows_vram = match value {
                            Value::String(s) => Some(s),
                            _ => {
                                return Err(serde::de::Error::custom("Invalid follows_vram value."))
                            }
                        };
                    }
                    Some("align") => {
                        align = parse_number(&value).map_err(serde::de::Error::custom)?;
                    }
                    Some("subalign") => {
                        subalign = parse_number(&value).map_err(serde::de::Error::custom)?;
                    }
                    Some("size") => {
                        size = parse_number(&value).map_err(serde::de::Error::custom)?;
                    }
                    Some("symbol_name_format") => {
                        symbol_name_format = match value {
                            Value::String(s) => Some(s),
                            _ => {
                                return Err(serde::de::Error::custom(
                                    "Invalid symbol_name_format value.",
                                ))
                            }
                        };
                    }
                    Some(other) => match args {
                        Some(ref mut a) => {
                            a.insert(other.to_string(), value);
                        }
                        None => {
                            let mut a = HashMap::new();
                            a.insert(other.to_string(), value);
                            args = Some(a);
                        }
                    },
                    None => {
                        return Err(serde::de::Error::custom(format!(
                            "Invalid segment property: {:?}",
                            key
                        )));
                    }
                }
            }
            Ok(YamlSegment {
                rom,
                segment_type: segment_type
                    .ok_or_else(|| serde::de::Error::custom("Segment type not specified"))?,
                name,
                dir,
                vram,
                subsegments,
                bss_size,
                vram_class,
                follows_vram,
                align,
                subalign,
                size,
                symbol_name_format,
                args: args.map(|a| YamlSegmentArgs::Dict(a)),
            })
        }
        Value::Sequence(l) => {
            let start = match l.get(0) {
                Some(value) => parse_address(value).map_err(serde::de::Error::custom)?,
                _ => return Err(serde::de::Error::custom("No value for segment at index 0.")),
            };

            let segment_type = match l.get(1) {
                Some(serde_yaml::Value::String(s)) => s.clone(),
                _ => "stub".to_string(),
            };

            let name = match l.get(2) {
                Some(serde_yaml::Value::String(s)) => s.clone(),
                _ => start
                    .map(|s| format!("{:X}", s))
                    .unwrap_or_else(|| "auto".to_string()), // Default name
            };

            let args: Option<Vec<Value>> = match l.len() > 3 {
                true => Some(l.iter().skip(3).cloned().collect()),
                false => None,
            };

            Ok(YamlSegment {
                rom: start,
                segment_type: segment_type,
                name: Some(name),
                dir: None,
                vram: None,
                bss_size: None,
                subsegments: None,
                vram_class: None,
                follows_vram: None,
                align: None,
                subalign: None,
                size: None,
                symbol_name_format: None,
                args: args.map(|a| YamlSegmentArgs::List(a)),
            })
        }
        _ => return Err(serde::de::Error::custom("Invalid segment")),
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum VramClass {
    Symbol(SymbolVramClass),
    Follows(FollowsVramClass),
    Hardcoded(HardcodedVramClass),
}

#[derive(Debug, Deserialize)]
pub struct SymbolVramClass {
    pub name: String,
    pub vram: u64,
    pub vram_symbol: String,
}

#[derive(Debug, Deserialize)]
pub struct FollowsVramClass {
    pub name: String,
    pub vram: u64,
    pub follows_classes: String,
}

#[derive(Debug, Deserialize)]
pub struct HardcodedVramClass {
    pub name: String,
    pub vram: u64,
}

pub fn load_yaml(path: &Path) -> Result<SplatYaml> {
    let yaml_data = std::fs::read_to_string(path)?;
    match serde_yaml::from_str(&yaml_data) {
        Ok(yaml) => Ok(yaml),
        Err(e) => Err(anyhow::anyhow!("Failed to parse YAML: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_yaml() {
        let path = Path::new("test_data/splat.yaml");
        load_yaml(path).unwrap();
    }
}
