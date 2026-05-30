use phf::phf_set;

pub static LEGAL_BLOCK_LIST: phf::Set<&'static str> = phf_set! {
    // Cloudflare Trust & Safety report 23d89ee107ed0942.
    "mms75/sfz",
};

#[cfg(test)]
mod tests {
    #[test]
    fn test_contain_phf() {
        let repo = "mms75/sfz".to_string();
        assert!(super::LEGAL_BLOCK_LIST.contains(repo.as_str()));
    }
}
