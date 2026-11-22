//! Küçük JSON değeri + yazıcı. Çıktı biçimi: 2 boşluk girinti, UTF-8 (ASCII kaçışı yok).
//! Aynı değer tipi CSV hücreleri için de kullanılır (True/False, boş = değer yok).

#[derive(Clone, Debug, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

pub fn obj(alanlar: Vec<(&str, Json)>) -> Json {
    Json::Obj(alanlar.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
}

/// Ondalık sayıyı en kısa geri-dönüşlü biçimde yazar; tam sayı değerli ise ".0" ekler (54 -> "54.0").
pub fn ondalik(f: f64) -> String {
    if f.is_nan() {
        "NaN".into()
    } else if f.is_infinite() {
        if f > 0.0 { "Infinity".into() } else { "-Infinity".into() }
    } else if f == f.trunc() && f.abs() < 1e16 {
        format!("{f:.1}")
    } else {
        format!("{f}")
    }
}

fn kacis(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}

fn girinti(out: &mut String, seviye: usize) {
    for _ in 0..seviye {
        out.push_str("  ");
    }
}

impl Json {
    pub fn guzel(&self) -> String {
        let mut s = String::new();
        self.yaz(&mut s, 0);
        s
    }

    fn yaz(&self, out: &mut String, sv: usize) {
        match self {
            Json::Null => out.push_str("null"),
            Json::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            Json::Int(i) => out.push_str(&i.to_string()),
            Json::Float(f) => out.push_str(&ondalik(*f)),
            Json::Str(s) => kacis(s, out),
            Json::Arr(v) if v.is_empty() => out.push_str("[]"),
            Json::Arr(v) => {
                out.push_str("[\n");
                for (i, x) in v.iter().enumerate() {
                    if i > 0 {
                        out.push_str(",\n");
                    }
                    girinti(out, sv + 1);
                    x.yaz(out, sv + 1);
                }
                out.push('\n');
                girinti(out, sv);
                out.push(']');
            }
            Json::Obj(v) if v.is_empty() => out.push_str("{}"),
            Json::Obj(v) => {
                out.push_str("{\n");
                for (i, (k, x)) in v.iter().enumerate() {
                    if i > 0 {
                        out.push_str(",\n");
                    }
                    girinti(out, sv + 1);
                    kacis(k, out);
                    out.push_str(": ");
                    x.yaz(out, sv + 1);
                }
                out.push('\n');
                girinti(out, sv);
                out.push('}');
            }
        }
    }

    /// CSV hücresi metni (tırnaklama csvw modülünde yapılır).
    pub fn hucre(&self) -> String {
        match self {
            Json::Null => String::new(),
            Json::Bool(b) => (if *b { "True" } else { "False" }).into(),
            Json::Int(i) => i.to_string(),
            Json::Float(f) => ondalik(*f),
            Json::Str(s) => s.clone(),
            other => other.guzel(),
        }
    }
}

impl From<&str> for Json {
    fn from(s: &str) -> Self {
        Json::Str(s.to_string())
    }
}
impl From<String> for Json {
    fn from(s: String) -> Self {
        Json::Str(s)
    }
}
impl From<&String> for Json {
    fn from(s: &String) -> Self {
        Json::Str(s.clone())
    }
}
impl From<bool> for Json {
    fn from(b: bool) -> Self {
        Json::Bool(b)
    }
}
impl From<i64> for Json {
    fn from(i: i64) -> Self {
        Json::Int(i)
    }
}
impl From<u32> for Json {
    fn from(i: u32) -> Self {
        Json::Int(i as i64)
    }
}
impl From<usize> for Json {
    fn from(i: usize) -> Self {
        Json::Int(i as i64)
    }
}
impl From<f64> for Json {
    fn from(f: f64) -> Self {
        Json::Float(f)
    }
}
impl<T: Into<Json>> From<Option<T>> for Json {
    fn from(o: Option<T>) -> Self {
        o.map_or(Json::Null, Into::into)
    }
}
impl<T: Into<Json>> From<Vec<T>> for Json {
    fn from(v: Vec<T>) -> Self {
        Json::Arr(v.into_iter().map(Into::into).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ondalik_bicimi() {
        assert_eq!(ondalik(54.0), "54.0");
        assert_eq!(ondalik(545000.0), "545000.0");
        assert_eq!(ondalik(2.72), "2.72");
        assert_eq!(ondalik(128.28), "128.28");
        assert_eq!(ondalik(29789.5), "29789.5");
    }

    #[test]
    fn girintili_yazim() {
        let j = obj(vec![
            ("a", 1i64.into()),
            ("b", vec![vec![1i64], vec![1, 4]].into()),
            ("c", Json::Arr(vec![])),
            ("d", "Ş\"\n".into()),
            ("e", Json::Null),
        ]);
        assert_eq!(
            j.guzel(),
            "{\n  \"a\": 1,\n  \"b\": [\n    [\n      1\n    ],\n    [\n      1,\n      4\n    ]\n  ],\n  \"c\": [],\n  \"d\": \"Ş\\\"\\n\",\n  \"e\": null\n}"
        );
    }
}
