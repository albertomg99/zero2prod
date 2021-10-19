//! tests/health_check.rs

// `actix_rt::test` is the testing equivalent of `actix_web::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// Use `cargo add actix-rt --dev --vers 2` to add `actix-rt`
// under `[dev-dependencies]` in Cargo.toml
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)

#[actix_rt::test]
async fn health_check_works() {
    //Arrange
    spawn_app().await().expect("Failed to spawn our app");

    // We need to bring in `reqwest`
// to perform HTTP requests against our application.
//
// Use `cargo add reqwest --dev --vers 0.11` to add
// it under `[dev-dependencies]` in Cargo.toml
let client = reqwest::Client::new();
}