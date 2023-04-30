use regex::Regex;
lazy_static::lazy_static! {
  static ref SQL_PATTERN: Regex = Regex::new("(;)|(-{2,})").unwrap();
}

pub fn check_sql_component(raw: &str) -> Result<String, Box<dyn std::error::Error>> {
    if SQL_PATTERN.is_match(raw) {
        Err(format!("`{raw}` is not a valid sql component").into())
    } else {
        Ok(raw.to_string())
    }
}
