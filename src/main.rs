use regex::Regex;

#[derive(Debug, Clone)]
pub struct MeetingLink {
    pub url: String,
    pub text: String, // we might not need this here
}

async fn get_month_meetings() -> anyhow::Result<Vec<MeetingLink>> {
    let client = reqwest::Client::new();

    let html_content = client
        .get("https://www.herne.de/allris/si010_j.asp?MM=11&YY=2024")
        .header(
            "Accept",
            "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
        )
        .header(
            "User-Agent",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:133.0) Gecko/20100101 Firefox/133.0",
        )
        .send()
        .await?
        .text()
        .await?;

    //let pattern = Regex::new(r#"href=["'](/allris/to010\.asp\?SILFDNR=[^"']+)["']"#)?;
    let pattern =
        Regex::new(r#"<a[^>]*href=["'](/allris/to010\.asp\?SILFDNR=[^"']+)["'][^>]*>(.*?)</a>"#)?;

    let links: Vec<MeetingLink> = pattern
        .captures_iter(&html_content)
        .filter_map(|cap| match (cap.get(1), cap.get(2)) {
            (Some(url), Some(text)) => Some(MeetingLink {
                url: url.as_str().to_string(),
                text: html_escape::decode_html_entities(text.as_str().trim()).to_string(),
            }),
            _ => None,
        })
        .collect();

    Ok(links)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let meetings = get_month_meetings().await?;

    for meeting in meetings {
        println!("{} - {}", meeting.url, meeting.text);
    }

    Ok(())
}
