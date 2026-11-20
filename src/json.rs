use std::collections::BTreeMap;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(String),
    Bool(bool),
    Null,
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
}

#[derive(Debug, Clone)]
pub struct SongJson {
    pub title: String,
    pub key: String,
    pub time_sig: String,
    pub author: String,
    pub tempo: String,
    pub hymn_number: String,
    pub presentation: String,
    pub lyrics: String,
    pub automatic: bool,
    pub slide_seconds: u64,
    pub next_song_delay_seconds: u64,
}

impl SongJson {
    pub fn to_json_string(&self) -> String {
        use crate::state::escape_json;
        format!(
            concat!(
                "{{\n",
                "  \"title\": \"{}\",\n",
                "  \"key\": \"{}\",\n",
                "  \"time_sig\": \"{}\",\n",
                "  \"author\": \"{}\",\n",
                "  \"tempo\": \"{}\",\n",
                "  \"hymn_number\": \"{}\",\n",
                "  \"presentation\": \"{}\",\n",
                "  \"lyrics\": \"{}\",\n",
                "  \"automatic\": {},\n",
                "  \"slide_seconds\": {},\n",
                "  \"next_song_delay_seconds\": {}\n",
                "}}"
            ),
            escape_json(&self.title),
            escape_json(&self.key),
            escape_json(&self.time_sig),
            escape_json(&self.author),
            escape_json(&self.tempo),
            escape_json(&self.hymn_number),
            escape_json(&self.presentation),
            escape_json(&self.lyrics),
            self.automatic,
            self.slide_seconds,
            self.next_song_delay_seconds
        )
    }
}

pub fn parse(text: &str) -> Result<Value, String> {
    let mut parser = Parser {
        bytes: text.as_bytes(),
        pos: 0,
    };
    let value = parser.value()?;
    parser.whitespace();
    if parser.pos != parser.bytes.len() {
        return Err(format!("Nadmiarowy tekst od pozycji {}", parser.pos));
    }
    Ok(value)
}

pub fn parse_song(text: &str) -> Result<SongJson, String> {
    let value = parse(text)?;
    let object = match value {
        Value::Object(object) => object,
        _ => return Err("Główną wartością JSON musi być obiekt".to_string()),
    };
    Ok(SongJson {
        title: string_field(&object, "title")?,
        key: optional_string(&object, "key"),
        time_sig: optional_string(&object, "time_sig"),
        author: optional_string(&object, "author"),
        tempo: optional_string(&object, "tempo"),
        hymn_number: optional_string(&object, "hymn_number"),
        presentation: optional_string(&object, "presentation"),
        lyrics: string_field(&object, "lyrics")?,
        automatic: bool_field(&object, "automatic").unwrap_or(false),
        slide_seconds: number_field(&object, "slide_seconds")
            .and_then(|n| n.parse::<u64>().ok())
            .unwrap_or(0),
        next_song_delay_seconds: number_field(&object, "next_song_delay_seconds")
            .and_then(|n| n.parse::<u64>().ok())
            .unwrap_or(3),
    })
}

fn string_field(object: &BTreeMap<String, Value>, name: &str) -> Result<String, String> {
    match object.get(name) {
        Some(Value::String(value)) => Ok(value.clone()),
        Some(_) => Err(format!("Pole {name} musi być tekstem")),
        None => Err(format!("Brak wymaganego pola {name}")),
    }
}

fn optional_string(object: &BTreeMap<String, Value>, name: &str) -> String {
    match object.get(name) {
        Some(Value::String(value)) => value.clone(),
        _ => String::new(),
    }
}

fn bool_field<'a>(object: &'a BTreeMap<String, Value>, name: &str) -> Option<bool> {
    match object.get(name) {
        Some(Value::Bool(value)) => Some(*value),
        _ => None,
    }
}

fn number_field<'a>(object: &'a BTreeMap<String, Value>, name: &str) -> Option<&'a str> {
    match object.get(name) {
        Some(Value::Number(value)) => Some(value),
        _ => None,
    }
}

struct Parser<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl Parser<'_> {
    fn whitespace(&mut self) {
        while self.pos < self.bytes.len()
            && matches!(self.bytes[self.pos], b' ' | b'\n' | b'\r' | b'\t')
        {
            self.pos += 1;
        }
    }

    fn value(&mut self) -> Result<Value, String> {
        self.whitespace();
        match self.peek() {
            Some(b'"') => Ok(Value::String(self.string()?)),
            Some(b'{') => self.object(),
            Some(b'[') => self.array(),
            Some(b't') => {
                self.keyword(b"true")?;
                Ok(Value::Bool(true))
            }
            Some(b'f') => {
                self.keyword(b"false")?;
                Ok(Value::Bool(false))
            }
            Some(b'n') => {
                self.keyword(b"null")?;
                Ok(Value::Null)
            }
            Some(b'-' | b'0'..=b'9') => self.number(),
            _ => Err(format!("Nieoczekiwana wartość przy pozycji {}", self.pos)),
        }
    }

    fn object(&mut self) -> Result<Value, String> {
        self.expect(b'{')?;
        self.whitespace();
        let mut fields = BTreeMap::new();
        if self.peek() == Some(b'}') {
            self.pos += 1;
            return Ok(Value::Object(fields));
        }
        loop {
            self.whitespace();
            let key = self.string()?;
            self.whitespace();
            self.expect(b':')?;
            let value = self.value()?;
            fields.insert(key, value);
            self.whitespace();
            match self.peek() {
                Some(b',') => self.pos += 1,
                Some(b'}') => {
                    self.pos += 1;
                    break;
                }
                _ => return Err(format!("Oczekiwano ',' lub '}}' przy pozycji {}", self.pos)),
            }
        }
        Ok(Value::Object(fields))
    }

    fn array(&mut self) -> Result<Value, String> {
        self.expect(b'[')?;
        self.whitespace();
        let mut values = Vec::new();
        if self.peek() == Some(b']') {
            self.pos += 1;
            return Ok(Value::Array(values));
        }
        loop {
            values.push(self.value()?);
            self.whitespace();
            match self.peek() {
                Some(b',') => self.pos += 1,
                Some(b']') => {
                    self.pos += 1;
                    break;
                }
                _ => return Err(format!("Oczekiwano ',' lub ']' przy pozycji {}", self.pos)),
            }
        }
        Ok(Value::Array(values))
    }

    fn string(&mut self) -> Result<String, String> {
        self.expect(b'"')?;
        let mut result = String::new();
        loop {
            let byte = self.peek().ok_or("Niedomknięty tekst JSON")?;
            match byte {
                b'"' => {
                    self.pos += 1;
                    return Ok(result);
                }
                b'\\' => {
                    self.pos += 1;
                    let escaped = self.peek().ok_or("Niepełna sekwencja escape")?;
                    self.pos += 1;
                    match escaped {
                        b'"' => result.push('"'),
                        b'\\' => result.push('\\'),
                        b'/' => result.push('/'),
                        b'b' => result.push('\u{0008}'),
                        b'f' => result.push('\u{000c}'),
                        b'n' => result.push('\n'),
                        b'r' => result.push('\r'),
                        b't' => result.push('\t'),
                        b'u' => {
                            let first = self.hex4()?;
                            let code = if (0xD800..=0xDBFF).contains(&first) {
                                if self.bytes.get(self.pos..self.pos + 2) != Some(b"\\u") {
                                    return Err("Brak drugiej części znaku Unicode".to_string());
                                }
                                self.pos += 2;
                                let second = self.hex4()?;
                                if !(0xDC00..=0xDFFF).contains(&second) {
                                    return Err("Nieprawidłowa para znaków Unicode".to_string());
                                }
                                0x10000
                                    + (((first as u32 - 0xD800) << 10)
                                        | (second as u32 - 0xDC00))
                            } else {
                                first as u32
                            };
                            result.push(
                                char::from_u32(code)
                                    .ok_or("Nieprawidłowy znak Unicode")?,
                            );
                        }
                        _ => return Err("Nieprawidłowa sekwencja escape".to_string()),
                    }
                }
                0x00..=0x1f => {
                    return Err("Niedozwolony znak sterujący w tekście JSON".to_string());
                }
                _ => {
                    let rest = std::str::from_utf8(&self.bytes[self.pos..])
                        .map_err(|_| "Nieprawidłowe UTF-8")?;
                    let ch = rest.chars().next().ok_or("Nieprawidłowy tekst")?;
                    result.push(ch);
                    self.pos += ch.len_utf8();
                }
            }
        }
    }

    fn hex4(&mut self) -> Result<u16, String> {
        if self.pos + 4 > self.bytes.len() {
            return Err("Niepełny znak Unicode".to_string());
        }
        let mut value = 0u16;
        for _ in 0..4 {
            let digit = match self.bytes[self.pos] {
                b'0'..=b'9' => self.bytes[self.pos] - b'0',
                b'a'..=b'f' => self.bytes[self.pos] - b'a' + 10,
                b'A'..=b'F' => self.bytes[self.pos] - b'A' + 10,
                _ => return Err("Nieprawidłowy zapis znaku Unicode".to_string()),
            };
            value = value * 16 + digit as u16;
            self.pos += 1;
        }
        Ok(value)
    }

    fn number(&mut self) -> Result<Value, String> {
        let start = self.pos;
        while matches!(
            self.peek(),
            Some(b'0'..=b'9' | b'-' | b'+' | b'.' | b'e' | b'E')
        ) {
            self.pos += 1;
        }
        let text = std::str::from_utf8(&self.bytes[start..self.pos])
            .map_err(|_| "Nieprawidłowy zapis liczby")?;
        text.parse::<f64>()
            .map_err(|_| format!("Nieprawidłowa liczba: {text}"))?;
        Ok(Value::Number(text.to_string()))
    }

    fn keyword(&mut self, keyword: &[u8]) -> Result<(), String> {
        if self.bytes.get(self.pos..self.pos + keyword.len()) == Some(keyword) {
            self.pos += keyword.len();
            Ok(())
        } else {
            Err(format!("Nieprawidłowa wartość przy pozycji {}", self.pos))
        }
    }

    fn expect(&mut self, expected: u8) -> Result<(), String> {
        self.whitespace();
        if self.peek() == Some(expected) {
            self.pos += 1;
            Ok(())
        } else {
            Err(format!(
                "Oczekiwano '{}' przy pozycji {}",
                expected as char, self.pos
            ))
        }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }
}
