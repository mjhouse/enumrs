use enumrs::Tagged;

#[derive(Tagged)]
pub enum Styles {
    #[tag(padding, 10)]
    #[tag(height, 100)]
    #[tag(width, 800)]
    #[tag(full_height, height + (padding * 2))]
    #[tag(full_width, width + (padding * 2))]
    Heading = 1,

    #[tag(padding, 10)]
    #[tag(height, 900)]
    #[tag(width, 800)]
    #[tag(full_height, height + (padding * 2))]
    #[tag(full_width, width + (padding * 2))]
    Content = 2,

    #[tag(padding, 10)]
    #[tag(height, 200)]
    #[tag(width, 800)]
    #[tag(full_height, height + (padding * 2))]
    #[tag(full_width, width + (padding * 2))]
    Footer = 3,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heading_simple_attributes() {
        let variant = Styles::Heading;
        assert_eq!(variant.padding(), Some(10));
        assert_eq!(variant.height(), Some(100));
        assert_eq!(variant.width(), Some(800));
    }

    #[test]
    fn test_heading_complex_attributes() {
        let variant = Styles::Heading;
        assert_eq!(variant.full_height(), Some(120));
        assert_eq!(variant.full_width(), Some(820));
    }

    #[test]
    fn test_content_complex_attributes() {
        let variant = Styles::Content;
        assert_eq!(variant.full_height(), Some(920));
        assert_eq!(variant.full_width(), Some(820));
    }
}
