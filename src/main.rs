slint::include_modules!();
pub mod structs;

fn main() -> Result<(), slint::PlatformError> {
    let app = App::new()?;
    app.run()
}
