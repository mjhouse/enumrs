use enumrs::Tagged;

#[derive(Tagged)]
pub enum Country {
    #[tag(value, 1)]
    #[tag(name, "Afghanistan")]
    #[tag(uuid, "c4448930-5269-11df-8e0a-080069138b88")]
    #[tag(description, "Afghanistan (AFG)")]
    AFG = 1,

    #[tag(value, 2)]
    #[tag(name, "Albania")]
    #[tag(uuid, "c44598ca-5269-11df-b8ba-080069138b88")]
    #[tag(description, "Albania (ALB)")]
    ALB = 2,

    #[tag(value, 3)]
    #[tag(name, "Algeria")]
    #[tag(uuid, "c446ab48-5269-11df-aec1-080069138b88")]
    #[tag(description, "Algeria (DZA)")]
    DZA = 3,

    #[tag(value, 281)]
    #[tag(name, "United States Minor Outlying Islands")]
    #[tag(uuid, "64d78320-495e-4602-83e9-c07e6ba55de5")]
    #[tag(description, "United States Minor Outlying Islands (UMI)")]
    UMI = 281,

    #[tag(value, 282)]
    #[tag(name, "Palestine, State of")]
    #[tag(uuid, "c1efc87a-ea2b-4aa4-8f1b-405245c83a42")]
    #[tag(description, "Palestine, State of (PSE)")]
    PSE = 282,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_attribute() {
        assert_eq!(Country::PSE.value(), Some(282));
        assert_eq!(Country::UMI.value(), Some(281));
    }

    #[test]
    fn test_name_attribute() {
        assert_eq!(Country::PSE.name(), Some("Palestine, State of"));
        assert_eq!(
            Country::UMI.name(),
            Some("United States Minor Outlying Islands")
        );
    }

    #[test]
    fn test_uuid_attribute() {
        assert_eq!(
            Country::PSE.uuid(),
            Some("c1efc87a-ea2b-4aa4-8f1b-405245c83a42")
        );
        assert_eq!(
            Country::UMI.uuid(),
            Some("64d78320-495e-4602-83e9-c07e6ba55de5")
        );
    }

    #[test]
    fn test_description_attribute() {
        assert_eq!(
            Country::PSE.description(),
            Some("Palestine, State of (PSE)")
        );
        assert_eq!(
            Country::UMI.description(),
            Some("United States Minor Outlying Islands (UMI)")
        );
    }
}
