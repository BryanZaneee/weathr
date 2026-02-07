pub struct AsciiDisplay;

impl AsciiDisplay {
    pub fn render_house() {
        let house = r#"
        /\
       /  \
      /    \
     /______\
     |  __  |
     | |  | |
     | |__| |
     |______|
"#;
        println!("{}", house);
    }
}
