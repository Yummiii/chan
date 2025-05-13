use chan_config::Config;

fn main() {
    println!("cargo:rerun-if-changed=config.toml");

    let config = Config::get();
    println!("cargo:rustc-env=DATABASE_URL={}", config.database.url);
}
