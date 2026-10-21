pub trait Transpile {
    fn to_html(&self) -> &str;
    fn to_pdf(&self) -> &str;
}


pub struct IR {
    // TODO: Define the intermediate representation
}

impl Transpile for IR {
    fn to_html(&self) -> &str {
        todo!();
    }

    fn to_pdf(&self) -> &str {
        todo!();
    }
}
