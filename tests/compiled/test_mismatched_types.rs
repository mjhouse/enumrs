use enumrs::Tagged;

#[derive(Tagged)]
pub enum TestEnum {

    #[tag(tagname)]
    Variant1 = 1,

    // Tags on different variants that have
    // different types will fail to compile
    // because we can't match and return the
    // values from the same function.
    #[tag(tagname, 3.5)]
    Variant2
}

pub fn main() {}