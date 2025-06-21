use enumrs::Tagged;

#[derive(Tagged)]
pub enum TestEnum {
    // The only variables that are in context 
    // while processing a tag expression are the
    // other tags that are on the same variant. 
    // #[tag(other, 10)]
    #[tag(value, other + 7)]
    Variant = 1,
}

pub fn main() {}