use lazy_static::lazy_static;
use regex::Regex;

// Apparently the wiki part of GitHub can also take mediawiki syntax!
// https://docs.github.com/en/communities/documenting-your-project-with-wikis/editing-wiki-content
// Transform them to pure markdown
// Transform images first, then links
pub fn github_wiki_markdown_to_pure_markdown<'a, 'b>(
    md: &'a str,
    account: &'b str,
    repo: &'b str,
) -> String {
    lazy_static! {
        static ref IMG_RE: Regex = Regex::new(
            "\\[\\[(?P<image_url>.*\\.(?i)(jpg|jpeg|png|gif))\\|(alt=)?(?P<link_text>.*?)\\]\\]"
        )
        .unwrap();
        static ref LINK_RE: Regex =
            Regex::new("\\[\\[(?P<link_text>.*?)\\| *(?P<page_name>.*?)\\]\\]").unwrap();
    }
    // Disregard alt for now.
    let processed_img_md = IMG_RE.replace_all(
        &md,
        format!(
            "![$link_text](https://raw.githubusercontent.com/wiki/{}/{}$image_url)",
            account, repo
        ),
    );
    LINK_RE
        .replace_all(
            &processed_img_md,
            format!("[$link_text](/{}/{}/wiki/$page_name)", account, repo),
        )
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_links() {
        let md = r#"[[/images/TimonHiWhite.jpg|Timon (Global), Tima (Swedish)]]"#;
        let result = github_wiki_markdown_to_pure_markdown(
            md,
            "Erithano",
            "Timon-Your-FAQ-bot-for-Microsoft-Teams",
        );
        assert_eq!(
            result,
            "![Timon (Global), Tima (Swedish)](https://raw.githubusercontent.com/wiki/Erithano/Timon-Your-FAQ-bot-for-Microsoft-Teams/images/TimonHiWhite.jpg)"
        );
    }

    #[test]
    fn sidebar_links() {
        let md = include_str!("../test-data/_Sidebar.md");
        let result = github_wiki_markdown_to_pure_markdown(
            md,
            "Erithano",
            "Timon-Your-FAQ-bot-for-Microsoft-Teams",
        );
        assert_eq!(result, include_str!("../test-data/_Sidebar_pure.md"));
    }
}
