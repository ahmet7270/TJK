//! Sistem libsqlite3 kitaplığına doğrudan C arayüzü (FFI). Dış paket yok.
//! Derleme: `-lsqlite3` (libsqlite3.so). Bu modül ana ikili, sorgu aracı ve testler tarafından paylaşılır.

#![allow(dead_code)]

use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::ptr;

#[repr(C)]
pub struct Sqlite3 {
    _p: [u8; 0],
}
#[repr(C)]
pub struct Stmt3 {
    _p: [u8; 0],
}

#[link(name = "sqlite3")]
extern "C" {
    fn sqlite3_libversion() -> *const c_char;
    fn sqlite3_open_v2(f: *const c_char, db: *mut *mut Sqlite3, flags: c_int, vfs: *const c_char) -> c_int;
    fn sqlite3_close_v2(db: *mut Sqlite3) -> c_int;
    fn sqlite3_exec(
        db: *mut Sqlite3,
        sql: *const c_char,
        cb: *const c_void,
        arg: *mut c_void,
        err: *mut *mut c_char,
    ) -> c_int;
    fn sqlite3_free(p: *mut c_void);
    fn sqlite3_errmsg(db: *mut Sqlite3) -> *const c_char;
    fn sqlite3_prepare_v2(
        db: *mut Sqlite3,
        sql: *const c_char,
        n: c_int,
        st: *mut *mut Stmt3,
        tail: *mut *const c_char,
    ) -> c_int;
    fn sqlite3_bind_text(st: *mut Stmt3, i: c_int, v: *const c_char, n: c_int, d: isize) -> c_int;
    fn sqlite3_bind_int64(st: *mut Stmt3, i: c_int, v: i64) -> c_int;
    fn sqlite3_bind_double(st: *mut Stmt3, i: c_int, v: f64) -> c_int;
    fn sqlite3_bind_null(st: *mut Stmt3, i: c_int) -> c_int;
    fn sqlite3_step(st: *mut Stmt3) -> c_int;
    fn sqlite3_reset(st: *mut Stmt3) -> c_int;
    fn sqlite3_clear_bindings(st: *mut Stmt3) -> c_int;
    fn sqlite3_finalize(st: *mut Stmt3) -> c_int;
    fn sqlite3_column_count(st: *mut Stmt3) -> c_int;
    fn sqlite3_column_name(st: *mut Stmt3, i: c_int) -> *const c_char;
    fn sqlite3_column_type(st: *mut Stmt3, i: c_int) -> c_int;
    fn sqlite3_column_int64(st: *mut Stmt3, i: c_int) -> i64;
    fn sqlite3_column_double(st: *mut Stmt3, i: c_int) -> f64;
    fn sqlite3_column_text(st: *mut Stmt3, i: c_int) -> *const u8;
    fn sqlite3_column_bytes(st: *mut Stmt3, i: c_int) -> c_int;
    fn sqlite3_last_insert_rowid(db: *mut Sqlite3) -> i64;
}

const SQLITE_OK: c_int = 0;
const SQLITE_ROW: c_int = 100;
const SQLITE_DONE: c_int = 101;
const OPEN_READWRITE: c_int = 0x2;
const OPEN_CREATE: c_int = 0x4;
const OPEN_READONLY: c_int = 0x1;
const SQLITE_TRANSIENT: isize = -1;

#[derive(Clone, Debug, PartialEq)]
pub enum Deger {
    Null,
    Int(i64),
    Real(f64),
    Text(String),
}

impl Deger {
    pub fn metin(&self) -> String {
        match self {
            Deger::Null => "NULL".into(),
            Deger::Int(i) => i.to_string(),
            Deger::Real(f) => f.to_string(),
            Deger::Text(s) => s.clone(),
        }
    }
    pub fn int(&self) -> Option<i64> {
        if let Deger::Int(i) = self { Some(*i) } else { None }
    }
}

impl From<&str> for Deger {
    fn from(s: &str) -> Self {
        Deger::Text(s.into())
    }
}
impl From<String> for Deger {
    fn from(s: String) -> Self {
        Deger::Text(s)
    }
}
impl From<&String> for Deger {
    fn from(s: &String) -> Self {
        Deger::Text(s.clone())
    }
}
impl From<i64> for Deger {
    fn from(i: i64) -> Self {
        Deger::Int(i)
    }
}
impl From<u32> for Deger {
    fn from(i: u32) -> Self {
        Deger::Int(i as i64)
    }
}
impl From<usize> for Deger {
    fn from(i: usize) -> Self {
        Deger::Int(i as i64)
    }
}
impl From<bool> for Deger {
    fn from(b: bool) -> Self {
        Deger::Int(b as i64)
    }
}
impl From<f64> for Deger {
    fn from(f: f64) -> Self {
        Deger::Real(f)
    }
}
impl<T: Into<Deger>> From<Option<T>> for Deger {
    fn from(o: Option<T>) -> Self {
        o.map_or(Deger::Null, Into::into)
    }
}

pub fn surum() -> String {
    unsafe { CStr::from_ptr(sqlite3_libversion()).to_string_lossy().into_owned() }
}

pub struct Db {
    p: *mut Sqlite3,
}

pub struct Ifade<'a> {
    p: *mut Stmt3,
    db: &'a Db,
}

impl Db {
    pub fn ac(yol: &str, salt_okunur: bool) -> Result<Db, String> {
        let c = CString::new(yol).map_err(|e| e.to_string())?;
        let mut p = ptr::null_mut();
        let flags = if salt_okunur { OPEN_READONLY } else { OPEN_READWRITE | OPEN_CREATE };
        let rc = unsafe { sqlite3_open_v2(c.as_ptr(), &mut p, flags, ptr::null()) };
        let db = Db { p };
        if rc != SQLITE_OK {
            return Err(format!("{yol} açılamadı: {}", db.hata()));
        }
        db.calistir("PRAGMA foreign_keys = ON;")?;
        Ok(db)
    }

    fn hata(&self) -> String {
        if self.p.is_null() {
            return "bellek yok".into();
        }
        unsafe { CStr::from_ptr(sqlite3_errmsg(self.p)).to_string_lossy().into_owned() }
    }

    /// Birden çok ifade içerebilen SQL metnini çalıştırır.
    pub fn calistir(&self, sql: &str) -> Result<(), String> {
        let c = CString::new(sql).map_err(|e| e.to_string())?;
        let mut err: *mut c_char = ptr::null_mut();
        let rc = unsafe { sqlite3_exec(self.p, c.as_ptr(), ptr::null(), ptr::null_mut(), &mut err) };
        if rc != SQLITE_OK {
            let m = if err.is_null() {
                self.hata()
            } else {
                let s = unsafe { CStr::from_ptr(err).to_string_lossy().into_owned() };
                unsafe { sqlite3_free(err as *mut c_void) };
                s
            };
            return Err(format!("SQL hatası: {m}"));
        }
        Ok(())
    }

    pub fn hazirla(&self, sql: &str) -> Result<Ifade<'_>, String> {
        let c = CString::new(sql).map_err(|e| e.to_string())?;
        let mut st = ptr::null_mut();
        let rc = unsafe { sqlite3_prepare_v2(self.p, c.as_ptr(), -1, &mut st, ptr::null_mut()) };
        if rc != SQLITE_OK {
            return Err(format!("hazırlama hatası: {} | {sql}", self.hata()));
        }
        Ok(Ifade { p: st, db: self })
    }

    /// Parametreli tek ifade; sonuç satırlarını döndürür.
    pub fn sorgu(&self, sql: &str, param: &[Deger]) -> Result<(Vec<String>, Vec<Vec<Deger>>), String> {
        let mut st = self.hazirla(sql)?;
        st.bagla(param)?;
        let kolonlar = st.kolonlar();
        let mut satirlar = Vec::new();
        while let Some(r) = st.adim()? {
            satirlar.push(r);
        }
        Ok((kolonlar, satirlar))
    }

    pub fn tek_int(&self, sql: &str, param: &[Deger]) -> Result<i64, String> {
        let (_, s) = self.sorgu(sql, param)?;
        s.first().and_then(|r| r.first()).and_then(Deger::int).ok_or_else(|| format!("tamsayı dönmedi: {sql}"))
    }

    pub fn son_rowid(&self) -> i64 {
        unsafe { sqlite3_last_insert_rowid(self.p) }
    }
}

impl Drop for Db {
    fn drop(&mut self) {
        unsafe {
            sqlite3_close_v2(self.p);
        }
    }
}

impl<'a> Ifade<'a> {
    pub fn bagla(&mut self, param: &[Deger]) -> Result<(), String> {
        unsafe {
            sqlite3_reset(self.p);
            sqlite3_clear_bindings(self.p);
        }
        for (i, d) in param.iter().enumerate() {
            let i = (i + 1) as c_int;
            let rc = unsafe {
                match d {
                    Deger::Null => sqlite3_bind_null(self.p, i),
                    Deger::Int(v) => sqlite3_bind_int64(self.p, i, *v),
                    Deger::Real(v) => sqlite3_bind_double(self.p, i, *v),
                    Deger::Text(s) => {
                        sqlite3_bind_text(self.p, i, s.as_ptr() as *const c_char, s.len() as c_int, SQLITE_TRANSIENT)
                    }
                }
            };
            if rc != SQLITE_OK {
                return Err(format!("bağlama hatası ({i}): {}", self.db.hata()));
            }
        }
        Ok(())
    }

    pub fn kolonlar(&self) -> Vec<String> {
        let n = unsafe { sqlite3_column_count(self.p) };
        (0..n).map(|i| unsafe { CStr::from_ptr(sqlite3_column_name(self.p, i)).to_string_lossy().into_owned() }).collect()
    }

    /// Bir adım ilerler; satır varsa döndürür.
    pub fn adim(&mut self) -> Result<Option<Vec<Deger>>, String> {
        let rc = unsafe { sqlite3_step(self.p) };
        match rc {
            SQLITE_ROW => {
                let n = unsafe { sqlite3_column_count(self.p) };
                let mut r = Vec::with_capacity(n as usize);
                for i in 0..n {
                    r.push(unsafe {
                        match sqlite3_column_type(self.p, i) {
                            1 => Deger::Int(sqlite3_column_int64(self.p, i)),
                            2 => Deger::Real(sqlite3_column_double(self.p, i)),
                            5 => Deger::Null,
                            _ => {
                                let t = sqlite3_column_text(self.p, i);
                                let n = sqlite3_column_bytes(self.p, i) as usize;
                                Deger::Text(String::from_utf8_lossy(std::slice::from_raw_parts(t, n)).into_owned())
                            }
                        }
                    });
                }
                Ok(Some(r))
            }
            SQLITE_DONE => Ok(None),
            _ => Err(format!("adım hatası: {}", self.db.hata())),
        }
    }

    /// Satır döndürmeyen ifadeyi (INSERT/UPDATE/DELETE) çalıştırır.
    pub fn yurut(&mut self, param: &[Deger]) -> Result<(), String> {
        self.bagla(param)?;
        while self.adim()?.is_some() {}
        Ok(())
    }
}

impl Drop for Ifade<'_> {
    fn drop(&mut self) {
        unsafe {
            sqlite3_finalize(self.p);
        }
    }
}
