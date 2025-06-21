use enumrs::Tagged;

#[derive(Tagged)]
pub enum TestEnum {
    // Tags must have a name. if no expression
    // is given, the value of the tag is a bool
    // set to `true`.
    #[tag()]
    Variant = 1,
}

pub fn main() {}