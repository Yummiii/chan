use chan_config::Config;

fn main() {
    if cfg!(debug_assertions) {
        println!("cargo:rerun-if-changed=config.toml");

        let config = Config::get();
        println!("cargo:rustc-env=DATABASE_URL={}", config.database.url);
    }
}
