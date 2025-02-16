use std::{collections::HashMap, str::FromStr};

/**
 * Configuration and configurability.
 */

use num_traits::{AsPrimitive, Float};
use num_integer::Integer;
use ultraviolet::Vec3;

#[derive(Debug, Default, Clone)]
pub enum ConfigValue {
    #[default]
    Empty,
    Text(String),
    Float(f32),
    Int(i64),
    IntPos(u64),
    Boolean(bool),
    Vector(Vec3),
}

impl ConfigValue {
    pub fn coerce_bool(&self) -> bool {
        match self {
            Self::Empty => false,
            Self::Text(text) => !text.is_empty(),
            &Self::Float(num) => num != 0.0,
            &Self::Int(num) => num != 0,
            &Self::IntPos(num) => num != 0,
            &Self::Boolean(whether) => whether,
            &Self::Vector(vec) => vec != Vec3::zero()
        }
    }
    
    pub fn coerce_int<I: Integer + Copy + FromStr + From<bool> + 'static>(&self) -> Result<I, &str> where f32: AsPrimitive<I>, i64: AsPrimitive<I>, u64: AsPrimitive<I> {
        match self {
            Self::Empty => Ok(I::zero()),
            Self::Text(text) => text.parse().map_err(|_| "Could not parse string as integer"),
            &Self::Float(num) => Ok(num.as_()),
            &Self::Int(num) => Ok(num.as_()),
            &Self::IntPos(num) => Ok(num.as_()),
            &Self::Boolean(whether) => Ok(whether.into()),
            &Self::Vector(vec) => Ok((vec.x as i64).as_())
        }
    }
    
    pub fn coerce_float<F: Float + Copy + FromStr + From<bool> + 'static>(&self) -> Result<F, &str> where f32: AsPrimitive<F>, i64: AsPrimitive<F>, u64: AsPrimitive<F> {
        match self {
            Self::Empty => Ok(F::zero()),
            Self::Text(text) => text.parse().map_err(|_| "Could not parse string as float"),
            &Self::Float(num) => Ok(num.as_()),
            &Self::Int(num) => Ok(num.as_()),
            &Self::IntPos(num) => Ok(num.as_()),
            &Self::Boolean(whether) => Ok(whether.into()),
            &Self::Vector(vec) => Ok(vec.x.as_())
        }
    }
    
    pub fn coerce_text(&self) -> String {
        match self {
            Self::Empty => "".to_string(),
            Self::Text(text) => text.clone(),
            &Self::Float(num) => num.to_string(),
            &Self::Int(num) => num.to_string(),
            &Self::IntPos(num) => num.to_string(),
            &Self::Boolean(whether) => whether.to_string(),
            Self::Vector(vec) => format!("{:?}", vec),
        }
    }
    
    pub fn coerce_vector(&self) -> Result<Vec3, &str> {
        match self {
            Self::Empty => Ok(Vec3::zero()),
            Self::Text(_text) => Err("Cannot coerce string to vector yet"),
            &Self::Float(num) => Ok(Vec3::new(num, 0.0, 0.0)),
            &Self::Int(num) => Ok(Vec3::new((num as i16).into(), 0.0, 0.0)),
            &Self::IntPos(num) => Ok(Vec3::new((num as u16).into(), 0.0, 0.0)),
            &Self::Boolean(whether) => Ok(Vec3::new(whether.into(), 0.0, 0.0)),
            &Self::Vector(vec) => Ok(vec),
        }
    }
}

pub trait Configurable {
    fn check_config_change(&mut self, config_name: &str, value: ConfigValue);
}

#[derive(Default, Clone)]
pub struct Config {
    config_values: HashMap<String, ConfigValue>,
}

impl Config {
    
}