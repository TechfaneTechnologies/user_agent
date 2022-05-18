use futures::future::join_all;
use scraper::{Html, Selector};

async fn fetch_data(routes: [&'static str; 6]) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    const LAST_KNOWN_HEADER: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.67 Safari/537.36 Edg/100.0.1185.39";
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "user-agent",
        reqwest::header::HeaderValue::from_static(LAST_KNOWN_HEADER),
    );
    let http_client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let base_url: &'static str = "https://www.whatismybrowser.com/guides/the-latest-user-agent/";
    let user_agent_list = join_all(
        routes
            .iter()
            .map(|route| async {
                http_client
                    .get(format!("{}{}", &base_url, *route))
                    .send()
                    .await
                    .map_err(|_| reqwest::StatusCode::INTERNAL_SERVER_ERROR)
                    .unwrap()
                    .text()
                    .await
                    .map_err(|_| reqwest::StatusCode::INTERNAL_SERVER_ERROR)
                    .unwrap()
            })
            .collect::<Vec<_>>(),
    )
    .await;
    Ok(user_agent_list)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let routes: [&'static str; 6] = ["windows", "macos", "edge", "chrome", "firefox", "safari"];
    let user_agent_list = fetch_data(routes).await?;
    let user_agent_list = user_agent_list
        .iter()
        .map(|user_agent_text| {
            let document = Html::parse_document(user_agent_text);
            let selector = Selector::parse(r#"table > tbody > tr > td > ul > li > span"#).unwrap();
            document
                .select(&selector)
                .into_iter()
                .map(|title| title.inner_html())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();
    println!("╭─ The Latest User Agents Are As Followings ─╮");
    println!("╰─                                            ─╯");
    println!(
        "╭─ Windows User Agents ─╮\n╰─{:#?}\n\n",
        user_agent_list[0]
    );
    println!("╭─ MacOs User Agents ─╮\n╰─{:#?}\n\n", user_agent_list[1]);
    println!(
        "╭─ Microsoft Edge User Agents ─╮\n╰─{:#?}\n\n",
        user_agent_list[2]
    );
    println!(
        "╭─ Google Chrome User Agents ─╮\n╰─{:#?}\n\n",
        user_agent_list[3]
    );
    println!(
        "╭─ Mozilla Firefox User Agents ─╮\n╰─{:#?}\n\n",
        user_agent_list[4]
    );
    println!(
        "╭─ Apple Safari User Agents ─╮\n╰─{:#?}\n\n",
        user_agent_list[5]
    );
    Ok(())
}
