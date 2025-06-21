use enumrs::Tagged;

#[derive(Tagged)]
pub enum TestEnum {
    // Tag names should be valid identifiers
    // in line with rust ident rules, which
    // means they can't begin with a number.
    #[tag(1padding)]
    Variant = 1,
}

pub fn main() {}