/* Any copyright is dedicated to the Public Domain.
 * http://creativecommons.org/publicdomain/zero/1.0/ */

#[cfg(test)]
mod tests {
    fn assert_is_lower_cased_self(domain: &str) {
        if let Ok(cow) = idna::domain_to_ascii_cow(domain.as_bytes(), idna::AsciiDenyList::URL) {
            assert_eq!(cow.as_ref(), domain.to_ascii_lowercase().as_str());
        } else {
            assert!(false);
        }
    }

    fn assert_is_err(domain: &str) {
        assert!(idna::domain_to_ascii_cow(domain.as_bytes(), idna::AsciiDenyList::URL).is_err());
    }

    #[test]
    fn ascii() {
        assert_is_lower_cased_self("example.com");
    }

    #[test]
    fn upper_case_ascii() {
        assert_is_lower_cased_self("EXAMPLE.COM");
    }

    #[test]
    fn mixed_case_ascii() {
        assert_is_lower_cased_self("eXAMPLE.cOM");
    }

    #[test]
    fn punycode_valid() {
        assert_is_lower_cased_self("xn--a-0fa.example");
    }

    #[test]
    fn punycode_valid_upper_case_ascii_part() {
        assert_is_lower_cased_self("xn--A-0fa.example");
    }

    #[test]
    fn punycode_valid_upper_case_prefix() {
        assert_is_lower_cased_self("XN--a-0fa.example");
    }

    #[test]
    fn punycode_valid_upper_case_punycode() {
        assert_is_lower_cased_self("xn--a-0FA.example");
    }

    #[test]
    fn punycode_upper_case_in_punycode_part() {
        // Not valid per UTS 46
        assert_is_lower_cased_self("xn--a-6da.example");
    }

    #[test]
    fn punycode_upper_case_in_punycode_part_mixed_case_prefix() {
        // Not valid per UTS 46
        assert_is_lower_cased_self("xN--a-6da.example");
    }

    #[test]
    fn punycode_invalid() {
        assert_is_err("xn--a-0.example");
    }

    #[test]
    fn unicode() {
        // Valid per UTS 46
        assert_is_err("aä.example");
    }

    #[test]
    fn bidi() {
        // Not valid per UTS 46
        assert_is_lower_cased_self("1.xn--mgbh0fb.2");
    }

    #[test]
    fn zwnj() {
        // Not valid per UTS 46
        assert_is_lower_cased_self("xn--ab-j1t.example");
    }

    #[test]
    fn zwj() {
        // Not valid per UTS 46
        assert_is_lower_cased_self("xn--ab-m1t.example");
    }
}
