//! Minimal deterministic JSON value and parser for MCP JSON-RPC 2.0.
//!
//! Milestone: M5 — Model-Agnostic MCP Play

use core::fmt;

/// Lightweight JSON value representation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum JsonValue {
  Null,
  Bool(bool),
  Number(i64),
  String(String),
  Array(Vec<JsonValue>),
  Object(Vec<(String, JsonValue)>),
}

impl JsonValue {
  /// Check if the value is null.
  pub const fn is_null(&self) -> bool {
    matches!(self, Self::Null)
  }

  /// Borrow as string if it is a string value.
  pub fn as_str(&self) -> Option<&str> {
    match self {
      Self::String(s) => Some(s.as_str()),
      _ => None,
    }
  }

  /// Extract number as i64 if it is numeric.
  pub const fn as_i64(&self) -> Option<i64> {
    match self {
      Self::Number(n) => Some(*n),
      _ => None,
    }
  }

  /// Extract boolean if it is a boolean.
  pub const fn as_bool(&self) -> Option<bool> {
    match self {
      Self::Bool(b) => Some(*b),
      _ => None,
    }
  }

  /// Borrow as array if it is an array.
  pub fn as_array(&self) -> Option<&[JsonValue]> {
    match self {
      Self::Array(a) => Some(a.as_slice()),
      _ => None,
    }
  }

  /// Borrow as object key-value slice if it is an object.
  pub fn as_object(&self) -> Option<&[(String, JsonValue)]> {
    match self {
      Self::Object(o) => Some(o.as_slice()),
      _ => None,
    }
  }

  /// Lookup a field by key in an object.
  pub fn get(&self, key: &str) -> Option<&JsonValue> {
    match self {
      Self::Object(entries) => entries.iter().find(|(k, _)| k == key).map(|(_, v)| v),
      _ => None,
    }
  }

  /// Serialize this JSON value to a compact JSON string.
  pub fn to_json_string(&self) -> String {
    let mut out = String::new();
    self.write_json(&mut out);
    out
  }

  fn write_json(&self, out: &mut String) {
    match self {
      Self::Null => out.push_str("null"),
      Self::Bool(b) => {
        if *b {
          out.push_str("true");
        } else {
          out.push_str("false");
        }
      }
      Self::Number(n) => out.push_str(&n.to_string()),
      Self::String(s) => {
        out.push('"');
        for c in s.chars() {
          match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            other => out.push(other),
          }
        }
        out.push('"');
      }
      Self::Array(arr) => {
        out.push('[');
        for (i, val) in arr.iter().enumerate() {
          if i > 0 {
            out.push(',');
          }
          val.write_json(out);
        }
        out.push(']');
      }
      Self::Object(obj) => {
        out.push('{');
        for (i, (k, val)) in obj.iter().enumerate() {
          if i > 0 {
            out.push(',');
          }
          out.push('"');
          for c in k.chars() {
            match c {
              '"' => out.push_str("\\\""),
              '\\' => out.push_str("\\\\"),
              '\n' => out.push_str("\\n"),
              '\r' => out.push_str("\\r"),
              '\t' => out.push_str("\\t"),
              other => out.push(other),
            }
          }
          out.push_str("\":");
          val.write_json(out);
        }
        out.push('}');
      }
    }
  }
}

impl fmt::Display for JsonValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(&self.to_json_string())
  }
}

/// Errors raised when parsing JSON text.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JsonParseError {
  pub position: usize,
  pub message: String,
}

impl fmt::Display for JsonParseError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "JSON parse error at {}: {}", self.position, self.message)
  }
}

/// Parse a JSON string into a [`JsonValue`].
pub fn parse_json(input: &str) -> Result<JsonValue, JsonParseError> {
  let chars: Vec<char> = input.chars().collect();
  let mut pos = 0;
  skip_whitespace(&chars, &mut pos);
  let value = parse_value(&chars, &mut pos)?;
  skip_whitespace(&chars, &mut pos);
  if pos < chars.len() {
    return Err(JsonParseError {
      position: pos,
      message: format!("unexpected trailing characters: '{}'", chars[pos]),
    });
  }
  Ok(value)
}

fn skip_whitespace(chars: &[char], pos: &mut usize) {
  while *pos < chars.len() && chars[*pos].is_whitespace() {
    *pos += 1;
  }
}

fn parse_value(chars: &[char], pos: &mut usize) -> Result<JsonValue, JsonParseError> {
  skip_whitespace(chars, pos);
  if *pos >= chars.len() {
    return Err(JsonParseError {
      position: *pos,
      message: "unexpected end of input".into(),
    });
  }

  match chars[*pos] {
    'n' => parse_null(chars, pos),
    't' | 'f' => parse_bool(chars, pos),
    '"' => parse_string(chars, pos).map(JsonValue::String),
    '[' => parse_array(chars, pos),
    '{' => parse_object(chars, pos),
    '-' | '0'..='9' => parse_number(chars, pos),
    other => Err(JsonParseError {
      position: *pos,
      message: format!("unexpected character: '{other}'"),
    }),
  }
}

fn parse_null(chars: &[char], pos: &mut usize) -> Result<JsonValue, JsonParseError> {
  if *pos + 4 <= chars.len()
    && chars[*pos] == 'n'
    && chars[*pos + 1] == 'u'
    && chars[*pos + 2] == 'l'
    && chars[*pos + 3] == 'l'
  {
    *pos += 4;
    Ok(JsonValue::Null)
  } else {
    Err(JsonParseError {
      position: *pos,
      message: "expected 'null'".into(),
    })
  }
}

fn parse_bool(chars: &[char], pos: &mut usize) -> Result<JsonValue, JsonParseError> {
  if *pos + 4 <= chars.len()
    && chars[*pos] == 't'
    && chars[*pos + 1] == 'r'
    && chars[*pos + 2] == 'u'
    && chars[*pos + 3] == 'e'
  {
    *pos += 4;
    Ok(JsonValue::Bool(true))
  } else if *pos + 5 <= chars.len()
    && chars[*pos] == 'f'
    && chars[*pos + 1] == 'a'
    && chars[*pos + 2] == 'l'
    && chars[*pos + 3] == 's'
    && chars[*pos + 4] == 'e'
  {
    *pos += 5;
    Ok(JsonValue::Bool(false))
  } else {
    Err(JsonParseError {
      position: *pos,
      message: "expected 'true' or 'false'".into(),
    })
  }
}

fn parse_string(chars: &[char], pos: &mut usize) -> Result<String, JsonParseError> {
  if *pos >= chars.len() || chars[*pos] != '"' {
    return Err(JsonParseError {
      position: *pos,
      message: "expected '\"'".into(),
    });
  }
  *pos += 1;
  let mut s = String::new();
  while *pos < chars.len() {
    let c = chars[*pos];
    if c == '"' {
      *pos += 1;
      return Ok(s);
    } else if c == '\\' {
      *pos += 1;
      if *pos >= chars.len() {
        return Err(JsonParseError {
          position: *pos,
          message: "unterminated escape sequence".into(),
        });
      }
      match chars[*pos] {
        '"' => s.push('"'),
        '\\' => s.push('\\'),
        '/' => s.push('/'),
        'b' => s.push('\x08'),
        'f' => s.push('\x0c'),
        'n' => s.push('\n'),
        'r' => s.push('\r'),
        't' => s.push('\t'),
        'u' => {
          // 4 hex digits
          if *pos + 4 >= chars.len() {
            return Err(JsonParseError {
              position: *pos,
              message: "invalid unicode escape".into(),
            });
          }
          let hex: String = chars[*pos + 1..=*pos + 4].iter().collect();
          if let Some(ch) = u16::from_str_radix(&hex, 16)
            .ok()
            .and_then(|code| char::from_u32(u32::from(code)))
          {
            s.push(ch);
          }
          *pos += 4;
        }
        other => s.push(other),
      }
    } else {
      s.push(c);
    }
    *pos += 1;
  }
  Err(JsonParseError {
    position: *pos,
    message: "unterminated string".into(),
  })
}

fn parse_number(chars: &[char], pos: &mut usize) -> Result<JsonValue, JsonParseError> {
  let start = *pos;
  if chars[*pos] == '-' {
    *pos += 1;
  }
  while *pos < chars.len() && chars[*pos].is_ascii_digit() {
    *pos += 1;
  }
  let slice: String = chars[start..*pos].iter().collect();
  match slice.parse::<i64>() {
    Ok(n) => Ok(JsonValue::Number(n)),
    Err(_) => Err(JsonParseError {
      position: start,
      message: format!("invalid numeric value: '{slice}'"),
    }),
  }
}

fn parse_array(chars: &[char], pos: &mut usize) -> Result<JsonValue, JsonParseError> {
  if *pos >= chars.len() || chars[*pos] != '[' {
    return Err(JsonParseError {
      position: *pos,
      message: "expected '['".into(),
    });
  }
  *pos += 1;
  let mut items = Vec::new();
  skip_whitespace(chars, pos);
  if *pos < chars.len() && chars[*pos] == ']' {
    *pos += 1;
    return Ok(JsonValue::Array(items));
  }

  loop {
    let item = parse_value(chars, pos)?;
    items.push(item);
    skip_whitespace(chars, pos);
    if *pos >= chars.len() {
      return Err(JsonParseError {
        position: *pos,
        message: "unterminated array; expected ']'".into(),
      });
    }
    if chars[*pos] == ']' {
      *pos += 1;
      return Ok(JsonValue::Array(items));
    } else if chars[*pos] == ',' {
      *pos += 1;
    } else {
      return Err(JsonParseError {
        position: *pos,
        message: format!("expected ',' or ']', found '{}'", chars[*pos]),
      });
    }
  }
}

fn parse_object(chars: &[char], pos: &mut usize) -> Result<JsonValue, JsonParseError> {
  if *pos >= chars.len() || chars[*pos] != '{' {
    return Err(JsonParseError {
      position: *pos,
      message: "expected '{'".into(),
    });
  }
  *pos += 1;
  let mut entries = Vec::new();
  skip_whitespace(chars, pos);
  if *pos < chars.len() && chars[*pos] == '}' {
    *pos += 1;
    return Ok(JsonValue::Object(entries));
  }

  loop {
    skip_whitespace(chars, pos);
    if *pos >= chars.len() || chars[*pos] != '"' {
      return Err(JsonParseError {
        position: *pos,
        message: "expected string key in object".into(),
      });
    }
    let key = parse_string(chars, pos)?;
    skip_whitespace(chars, pos);
    if *pos >= chars.len() || chars[*pos] != ':' {
      return Err(JsonParseError {
        position: *pos,
        message: "expected ':' after object key".into(),
      });
    }
    *pos += 1;
    let value = parse_value(chars, pos)?;
    entries.push((key, value));
    skip_whitespace(chars, pos);
    if *pos >= chars.len() {
      return Err(JsonParseError {
        position: *pos,
        message: "unterminated object; expected '}'".into(),
      });
    }
    if chars[*pos] == '}' {
      *pos += 1;
      return Ok(JsonValue::Object(entries));
    } else if chars[*pos] == ',' {
      *pos += 1;
    } else {
      return Err(JsonParseError {
        position: *pos,
        message: format!("expected ',' or '}}', found '{}'", chars[*pos]),
      });
    }
  }
}
