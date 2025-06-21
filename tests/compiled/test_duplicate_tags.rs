use enumrs::Tagged;

#[derive(Tagged)]
pub enum TestEnum {
    // Tag names should be unique for the
    // variant that they're on.
    #[tag(tagname)]
    #[tag(tagname)]
    Variant = 1,
}

pub fn main() {}