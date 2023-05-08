use dialoguer::{theme::ColorfulTheme, Input};

fn main() {
    let sss: String = Input::with_theme(&ColorfulTheme::default())
        // .with_prompt("it works fine")
        .with_prompt("it do not \n work fine")
        .with_initial_text("ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890")
        .interact_text()
        .unwrap();

    println!("{:?}", sss);
}
