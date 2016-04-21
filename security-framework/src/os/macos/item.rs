use core_foundation::base::TCFType;
use core_foundation::string::CFString;
use security_framework_sys::item::*;

use os::macos::PrivKeyType;

#[derive(Debug, Copy, Clone)]
pub enum KeyType {
    Rsa,
    Dsa,
    Aes,
    Des,
    TripleDes,
    Rc4,
    Cast,
    #[cfg(feature = "OSX_10_9")]
    Ec,
}

impl PrivKeyType for KeyType {
    fn to_str(&self) -> CFString {
        let raw = match *self {
            KeyType::Rsa => kSecAttrKeyTypeRSA,
            KeyType::Dsa => kSecAttrKeyTypeDSA,
            KeyType::Aes => kSecAttrKeyTypeAES,
            KeyType::Des => kSecAttrKeyTypeDES,
            KeyType::TripleDes => kSecAttrKeyType3DES,
            KeyType::Rc4 => kSecAttrKeyTypeRC4,
            KeyType::Cast => kSecAttrKeyTypeCAST,
            #[cfg(feature = "OSX_10_9")]
            KeyType::Ec => kSecAttrKeyTypeEC,
        };
        unsafe { CFString::wrap_under_get_rule(raw) }
    }
}

#[cfg(test)]
mod test {
    use tempdir::TempDir;

    use item::*;
    use os::macos::certificate::SecCertificateExt;
    use os::macos::test::keychain;

    #[test]
    fn find_certificate() {
        let dir = p!(TempDir::new("find_certificate"));
        let keychain = keychain(dir.path());
        let results = p!(ItemSearchOptions::new()
                             .keychains(&[keychain])
                             .class(ItemClass::Certificate)
                             .search());
        assert_eq!(1, results.len());
        let certificate = match results[0].reference {
            Some(Reference::Certificate(ref cert)) => cert,
            _ => panic!("expected certificate"),
        };
        assert_eq!("foobar.com", p!(certificate.common_name()).to_string());
    }
}
