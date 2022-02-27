use std::env;
use dotenv::dotenv;
use webhook::client::{WebhookClient, WebhookResult};
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() -> WebhookResult<()> {
    // dotenv().expect(".env file not found");
    let url = env::var("DISCORD_WEBHOOK").expect("Expected a token in the environment");
    let client = WebhookClient::new(&url);
    println!("Hello, world!");
    let links = ["https://api.foxsports.com/v1/rss", "https://sports.ndtv.com/rss/football"];
    let mut guids:[String; 2] = ["".to_string(),"".to_string()];
    let mut interval = time::interval(Duration::from_millis(60000));

    loop {
        for numbo in 0..2 {
            let body = reqwest::get(links[numbo])
                .await?
                .text()
                .await?;
            let guid_pre: Vec<String> = body.split("<guid").map(String::from).collect();
            let guid_pre2: Vec<String> = guid_pre[1].split(">").map(String::from).collect();
            let guid_pre3: Vec<String> = guid_pre2[1].split("</guid").map(String::from).collect();
            let guid = &guid_pre3[0];
            println!("{:#}", guid);
            if guid.to_string() != guids[numbo] {
                guids[numbo] = guid.to_string();
                let item_pre: Vec<String> = body.split("<item>").map(String::from).collect();
                let item_pre2: Vec<String> = item_pre[1].split("</item>").map(String::from).collect();
                let item = item_pre2[0].to_string();
                let main_title_pre: Vec<String> = body.split("<title>").map(String::from).collect();
                let main_title_pre2: Vec<String> = main_title_pre[1].split("</title").map(String::from).collect();
                let main_title = main_title_pre2[0].to_string();
                let link_pre: Vec<String> = item.split("<link>").map(String::from).collect();
                let link_pre2: Vec<String> = link_pre[1].split("</link").map(String::from).collect();
                let link: String;
                if link_pre2[0].to_string().contains("\t"){
                    let link_pre3: Vec<String> = link_pre2[0].split("\th").map(String::from).collect();
                    let link_pre4: Vec<String> = link_pre3[1].split("\n").map(String::from).collect();
                    link = "h".to_owned() + &link_pre4[0].to_string();
                }
                else{
                    link = link_pre2[0].to_string();
                }
                let title_pre: Vec<String> = item.split("<title>").map(String::from).collect();
                let title_pre2: Vec<String> = title_pre[1].split("</title").map(String::from).collect();
                let mut title: String;
                if title_pre2[0].to_string().contains("CDATA"){
                    let title_pre3: Vec<String> = item.split("CDATA[").map(String::from).collect();
                    let title_pre4: Vec<String> = title_pre3[1].split("]").map(String::from).collect();
                    title = title_pre4[0].to_string();
                }
                else{
                    title = title_pre2[0].to_string();
                }
                let desc_pre: Vec<String> = item.split("<description>").map(String::from).collect();
                let desc_pre2: Vec<String> = desc_pre[1].split("</description>").map(String::from).collect();
                let mut desc: String;
                if desc_pre2[0].to_string().contains("CDATA"){
                    let desc_pre3: Vec<String> = item.split("CDATA[").map(String::from).collect();
                    let desc_pre4: Vec<String> = desc_pre3[1].split("]").map(String::from).collect();
                    desc = desc_pre4[0].to_string();
                }
                else{
                    desc = desc_pre2[0].to_string();
                }
                desc = desc.replace("&#8217;", "'");
                desc = desc.replace("&amp;", "");
                title = title.replace("&#8217;", "'");
                title = title.replace("&amp;", "");
                client.send(|message| message
                    .username("In Sports News!")
                    .avatar_url("https://hackathon.kavinplays.ml/favicon.png")
                    .embed(|embed| embed
                        .title(&title)
                        .color("14177041")
                        .url(&link)
                        .field(&main_title, &desc, false))).await?;
            }
        interval.tick().await;
        }
    }
}