use anyhow::{anyhow, Result};

pub fn parse_duration(duration_text: &str) -> Result<f64> {
    return match duration_text.split_whitespace().collect::<Vec<_>>()[..] {
        [t, "min"] => Ok(t.parse::<f64>()? * 60.),
        [t, "s"] => Ok(t.parse::<f64>()?),
        _ => Err(anyhow!("failed to parse duration")),
    };
}
