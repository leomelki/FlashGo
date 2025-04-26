use std::collections::HashMap;

pub type AnimationConfig = HashMap<String, AnimationConfigValue>;

pub enum AnimationConfigValue {
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
}

impl From<&AnimationConfigValue> for i32 {
    fn from(value: &AnimationConfigValue) -> Self {
        match value {
            AnimationConfigValue::Int(i) => *i,
            AnimationConfigValue::Float(f) => *f as i32,
            AnimationConfigValue::String(s) => s.parse::<i32>().unwrap(),
            AnimationConfigValue::Bool(b) => *b as i32,
        }
    }
}

impl From<&AnimationConfigValue> for f32 {
    fn from(value: &AnimationConfigValue) -> Self {
        match value {
            AnimationConfigValue::Float(f) => *f,
            AnimationConfigValue::Int(i) => *i as f32,
            AnimationConfigValue::String(s) => s.parse::<f32>().unwrap(),
            AnimationConfigValue::Bool(b) => *b as i32 as f32,
        }
    }
}

impl From<&AnimationConfigValue> for String {
    fn from(value: &AnimationConfigValue) -> Self {
        match value {
            AnimationConfigValue::String(s) => s.clone(),
            AnimationConfigValue::Int(i) => i.to_string(),
            AnimationConfigValue::Float(f) => f.to_string(),
            AnimationConfigValue::Bool(b) => b.to_string(),
        }
    }
}

impl From<&AnimationConfigValue> for bool {
    fn from(value: &AnimationConfigValue) -> Self {
        match value {
            AnimationConfigValue::Bool(b) => *b,
            AnimationConfigValue::Int(i) => *i != 0,
            AnimationConfigValue::Float(f) => *f != 0.0,
            AnimationConfigValue::String(s) => s.parse::<bool>().unwrap(),
        }
    }
}
