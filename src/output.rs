#[derive(Debug)]
pub struct Rezzy {
    pub stop_light: char,
    pub message: String,
}

impl Rezzy {
    pub fn build_output(&self) {
        println!("{} -- {}", self.stop_light, self.message);
    }

    pub fn new(result: char, message: String) -> Rezzy {
        Rezzy {
            stop_light: result,
            message: message,
        }
    }

}
