#[cxx::bridge(namespace = "ns")]
pub mod ffi {
    extern "Rust" {
        type DB;
        fn create_db(path: &str) -> Box<DB>;
        fn nocrash(self: &DB) -> &str;
        fn crash(self: &DB) -> String;
    }
}

pub struct DB {}

impl DB {
    pub fn new() -> Self {
        DB {}
    }

    pub fn nocrash(&self) -> &str {
        "nocrash called"
    }

    pub fn crash(&self) -> String {
        "crash called".to_string()
    }
}

pub fn create_db(path: &str) -> Box<DB> {
    println!("create_db called {path}");
    let db = DB::new();
    Box::new(db)
}
