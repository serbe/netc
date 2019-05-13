pub struct Config {
    pub db: String,
    pub server: String,
    pub target: String,
    pub workers: usize,
}

pub fn get_config() -> Config {
    let db = dotenv::var("db")
        .expect("No found variable db like postgres://postgres@localhost:5433 in environment");
    let server =
        dotenv::var("server").expect("No found variable server like 0.0.0.0:8080 in environment");
    let target = dotenv::var("target")
        .expect("No found variable target like http://targethost:433/path in environment");
    let workers = dotenv::var("workers")
        .expect("No found variable workers like 4 in environment")
        .parse::<usize>()
        .expect("wrong variable workers in environment");
    Config {
        db,
        server,
        target,
        workers,
    }
}
