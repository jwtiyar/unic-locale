use unic_langid_impl::LanguageIdentifier;
use unic_locale_impl::extensions::UnicodeExtensionKey;
use unic_locale_impl::parser::parse_locale;
use unic_locale_impl::{ExtensionsMap, Locale};

fn assert_locale_extensions(loc: &Locale, extensions: &ExtensionsMap) {
    assert_eq!(&loc.extensions, extensions);
}

fn assert_parsed_locale_identifier(input: &str, extensions: &ExtensionsMap) {
    let loc = parse_locale(input).unwrap();
    assert_locale_extensions(&loc, extensions);
}

#[test]
fn test_basic() {
    let loc: Locale = "en-US".parse().unwrap();
    let loc2 = Locale {
        langid: LanguageIdentifier::from_parts(Some("en"), None, Some("US"), None).unwrap(),
        extensions: ExtensionsMap::default(),
    };
    assert_eq!(loc, loc2);
}

#[test]
fn test_from_parts() {
    let extensions = ExtensionsMap::default();
    let loc = Locale::from_parts(Some("en"), None, None, None, Some(extensions)).unwrap();
    let loc2 = Locale {
        langid: LanguageIdentifier::from_parts(Some("en"), None, None, None).unwrap(),
        extensions: ExtensionsMap::default(),
    };
    assert_eq!(loc, loc2);
}

#[test]
fn test_locale_identifier() {
    let mut extensions = ExtensionsMap::default();
    extensions
        .set_unicode_value(UnicodeExtensionKey::HourCycle, Some("h12"))
        .unwrap();
    assert_parsed_locale_identifier("pl-u-hc-h12", &extensions);

    let mut extensions = ExtensionsMap::default();
    extensions.set_private_value("testing", None).unwrap();
    assert_parsed_locale_identifier("und-x-testing", &extensions);
}

#[test]
fn test_serialize_locale() {
    let loc: Locale = "en-u-hc-h12".parse().unwrap();
    assert_eq!(&loc.to_string(), "en-u-hc-h12");
}

#[test]
fn test_from_langid() {
    let langid: LanguageIdentifier = "en-US".parse().unwrap();
    let loc = Locale::from(langid);
    assert_eq!(&loc.to_string(), "en-US");
}

#[test]
fn test_to_langid() {
    let loc: Locale = "en-US-u-hc-h12".parse().unwrap();
    let langid: LanguageIdentifier = loc.into();
    assert_eq!(langid.to_string(), "en-US");
}

#[test]
fn test_from_parts_unchecked() {
    let loc: Locale = Locale::from_parts_unchecked(Some("en"), None, Some("US"), None, None);
    assert_eq!(loc.to_string(), "en-US");
}

#[test]
fn test_matches() {
    let loc_en: Locale = "en-u-hc-h12".parse().unwrap();
    let loc_en_us: Locale = "en-US".parse().unwrap();
    let loc_en_us2: Locale = "en-US-u-hc-h24".parse().unwrap();
    let loc_pl: Locale = "pl".parse().unwrap();
    assert_eq!(loc_en.matches(&loc_en_us, false, false), false);
    assert_eq!(loc_en_us.matches(&loc_en_us2, false, false), true);
    assert_eq!(loc_en.matches(&loc_pl, false, false), false);
    assert_eq!(loc_en.matches(&loc_en_us, true, false), true);

    let langid_en: LanguageIdentifier = "en-US".parse().unwrap();
    assert_eq!(langid_en.matches(&loc_en_us, true, true), true);
    assert_eq!(
        loc_en_us.matches(&Locale::from(langid_en), true, true),
        true
    );
}

#[test]
fn test_set_fields() {
    let mut loc = Locale::default();
    assert_eq!(&loc.to_string(), "und");

    loc.set_language(Some("pl"))
        .expect("Setting language failed");
    assert_eq!(&loc.to_string(), "pl");

    loc.set_language(Some("de"))
        .expect("Setting language failed");
    assert_eq!(&loc.to_string(), "de");
    loc.set_region(Some("AT")).expect("Setting region failed");
    assert_eq!(&loc.to_string(), "de-AT");
    loc.set_script(Some("Latn")).expect("Setting script failed");
    assert_eq!(&loc.to_string(), "de-Latn-AT");
    loc.set_variants(&["macos"])
        .expect("Setting variants failed");
    assert_eq!(&loc.to_string(), "de-Latn-AT-macos");

    loc.set_language(None).expect("Setting language failed");
    assert_eq!(&loc.to_string(), "und-Latn-AT-macos");
    loc.set_region(None).expect("Setting region failed");
    assert_eq!(&loc.to_string(), "und-Latn-macos");
    loc.set_script(None).expect("Setting script failed");
    assert_eq!(&loc.to_string(), "und-macos");
    loc.set_variants(&[]).expect("Setting variants failed");
    assert_eq!(&loc.to_string(), "und");
}