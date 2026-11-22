//! tjk_pilot.sqlite için salt okunur sorgu aracı (sistemde sqlite3 komut satırı aracı olmadığı için).
//! Kullanım: tjk_sorgu VERITABANI.sqlite "SQL" ["SQL" ...]

#[path = "../sqlite.rs"]
mod sqlite;

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if a.len() < 2 {
        eprintln!("Kullanım: tjk_sorgu VERITABANI.sqlite \"SQL\" [\"SQL\" ...]");
        std::process::exit(2);
    }
    let db = match sqlite::Db::ac(&a[0], true) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("HATA: {e}");
            std::process::exit(1);
        }
    };
    for sql in &a[1..] {
        println!("sql> {}", sql.split_whitespace().collect::<Vec<_>>().join(" "));
        match db.sorgu(sql, &[]) {
            Ok((k, satirlar)) => {
                let mut gen: Vec<usize> = k.iter().map(|x| x.chars().count()).collect();
                let metin: Vec<Vec<String>> = satirlar.iter().map(|r| r.iter().map(|d| d.metin()).collect()).collect();
                for r in &metin {
                    for (i, h) in r.iter().enumerate() {
                        gen[i] = gen[i].max(h.chars().count());
                    }
                }
                let yaz = |r: &[String]| {
                    r.iter().enumerate().map(|(i, h)| format!("{h:<w$}", w = gen[i])).collect::<Vec<_>>().join(" | ")
                };
                println!("{}", yaz(&k));
                println!("{}", gen.iter().map(|w| "-".repeat(*w)).collect::<Vec<_>>().join("-+-"));
                for r in &metin {
                    println!("{}", yaz(r));
                }
                println!("({} satır)\n", metin.len());
            }
            Err(e) => {
                eprintln!("HATA: {e}");
                std::process::exit(1);
            }
        }
    }
}
