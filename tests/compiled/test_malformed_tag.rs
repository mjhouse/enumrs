use enumrs::Tagged;

#[derive(Tagged)]
pub enum TestEnum {
    // Tags that have expressions must be delimited
    // with a comma.
    #[tag(tagname 10)]
    Variant = 1,
}

pub fn main() {}