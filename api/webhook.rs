use vercel_runtime::{run, Body, Error, Request, RequestExt, Response};
use serde_json::{json, Value};
use std::collections::HashMap;
use chrono::{DateTime, Datelike};
use teloxide::types::{Update, UpdateKind, Message, User, Chat, ForwardedFrom};

// Copy the age data function from main.rs
fn get_age_data() -> HashMap<u64, u64> {
    let mut ages = HashMap::new();
    ages.insert(2768409, 1383264000000);
    ages.insert(7679610, 1388448000000);
    ages.insert(11538514, 1391212000000);
    ages.insert(15835244, 1392940000000);
    ages.insert(23646077, 1393459000000);
    ages.insert(38015510, 1393632000000);
    ages.insert(44634663, 1399334000000);
    ages.insert(46145305, 1400198000000);
    ages.insert(54845238, 1411257000000);
    ages.insert(63263518, 1414454000000);
    ages.insert(101260938, 1425600000000);
    ages.insert(101323197, 1426204000000);
    ages.insert(111220210, 1429574000000);
    ages.insert(103258382, 1432771000000);
    ages.insert(103151531, 1433376000000);
    ages.insert(116812045, 1437696000000);
    ages.insert(122600695, 1437782000000);
    ages.insert(109393468, 1439078000000);
    ages.insert(112594714, 1439683000000);
    ages.insert(124872445, 1439856000000);
    ages.insert(130029930, 1441324000000);
    ages.insert(125828524, 1444003000000);
    ages.insert(133909606, 1444176000000);
    ages.insert(157242073, 1446768000000);
    ages.insert(143445125, 1448928000000);
    ages.insert(148670295, 1452211000000);
    ages.insert(152079341, 1453420000000);
    ages.insert(171295414, 1457481000000);
    ages.insert(181783990, 1460246000000);
    ages.insert(222021233, 1465344000000);
    ages.insert(225034354, 1466208000000);
    ages.insert(278941742, 1473465000000);
    ages.insert(285253072, 1476835000000);
    ages.insert(294851037, 1479600000000);
    ages.insert(297621225, 1481846000000);
    ages.insert(328594461, 1482969000000);
    ages.insert(337808429, 1487707000000);
    ages.insert(341546272, 1487782000000);
    ages.insert(352940995, 1487894000000);
    ages.insert(369669043, 1490918000000);
    ages.insert(400169472, 1501459000000);
    ages.insert(805158066, 1563208000000);
    ages.insert(1974255900, 1634000000000);
    ages
}

fn get_age_estimate(user_id: u64) -> (String, String) {
    let ages = get_age_data();
    let mut sorted_ids: Vec<u64> = ages.keys().cloned().collect();
    sorted_ids.sort();
    
    let min_id = sorted_ids[0];
    let max_id = sorted_ids[sorted_ids.len() - 1];
    
    if user_id < min_id {
        let date = DateTime::from_timestamp_millis(ages[&min_id] as i64).unwrap_or_default();
        return ("older_than".to_string(), format!("{}/{}", date.month(), date.year()));
    } else if user_id > max_id {
        let date = DateTime::from_timestamp_millis(ages[&max_id] as i64).unwrap_or_default();
        return ("newer_than".to_string(), format!("{}/{}", date.month(), date.year()));
    } else {
        let mut lid = sorted_ids[0];
        for &nid in &sorted_ids {
            if user_id <= nid {
                let lage = ages[&lid];
                let uage = ages[&nid];
                
                let id_ratio = (user_id - lid) as f64 / (nid - lid) as f64;
                let mid_date = (id_ratio * (uage - lage) as f64 + lage as f64) as i64;
                
                let date = DateTime::from_timestamp_millis(mid_date).unwrap_or_default();
                return ("approx".to_string(), format!("{}/{}", date.month(), date.year()));
            } else {
                lid = nid;
            }
        }
    }
    
    ("unknown".to_string(), "unknown".to_string())
}

fn format_user_info(user: &User, title: &str) -> String {
    let mut info = format!("👤 {}\n", title);
    info.push_str(&format!(" ├ id: {}\n", user.id));
    info.push_str(&format!(" ├ is_bot: {}\n", if user.is_bot { "true" } else { "false" }));
    info.push_str(&format!(" ├ first_name: {}\n", user.first_name));
    
    if let Some(last_name) = &user.last_name {
        info.push_str(&format!(" ├ last_name: {}\n", last_name));
    }
    
    if let Some(username) = &user.username {
        info.push_str(&format!(" ├ username: {}\n", username));
    }
    
    if let Some(language_code) = &user.language_code {
        info.push_str(&format!(" ├ language_code: {} (-)\n", language_code));
    }
    
    let (age_type, date) = get_age_estimate(user.id.0);
    let age_symbol = match age_type.as_str() {
        "older_than" | "newer_than" => "?",
        _ => "?"
    };
    info.push_str(&format!(" └ created: {} {} ({})\n", age_type, date, age_symbol));
    
    info
}

fn format_chat_info(chat: &Chat) -> String {
    let mut info = String::new();
    
    let chat_type = match &chat.kind {
        teloxide::types::ChatKind::Private(_) => "private",
        teloxide::types::ChatKind::Public(public_chat) => {
            match public_chat.kind {
                teloxide::types::PublicChatKind::Group(_) => "group",
                teloxide::types::PublicChatKind::Supergroup(_) => "supergroup", 
                teloxide::types::PublicChatKind::Channel(_) => "channel",
            }
        }
    };
    
    info.push_str(&format!("💬 Chat\n"));
    info.push_str(&format!(" ├ id: {}\n", chat.id));
    
    if let Some(title) = chat.title() {
        info.push_str(&format!(" ├ type: {}\n", chat_type));
        info.push_str(&format!(" ├ title: {}\n", title));
        if let Some(username) = chat.username() {
            info.push_str(&format!(" └ username: {}\n", username));
        } else {
            info = info.replace(" ├ title:", " └ title:");
        }
    } else {
        if let Some(username) = chat.username() {
            info.push_str(&format!(" ├ type: {}\n", chat_type));
            info.push_str(&format!(" └ username: {}\n", username));
        } else {
            info.push_str(&format!(" └ type: {}\n", chat_type));
        }
    }
    
    info
}

fn format_forwarded_from_info(forwarded_from: &ForwardedFrom) -> String {
    match forwarded_from {
        ForwardedFrom::User(user) => {
            format_user_info(user, "Forwarded from")
        }
        ForwardedFrom::SenderName(name) => {
            format!("👤 Forwarded from\n └ sender_name: {}\n", name)
        }
        ForwardedFrom::Chat(chat) => {
            let title = chat.title().unwrap_or("Unknown");
            let chat_type = match &chat.kind {
                teloxide::types::ChatKind::Private(_) => "private",
                teloxide::types::ChatKind::Public(public_chat) => {
                    match public_chat.kind {
                        teloxide::types::PublicChatKind::Group(_) => "group",
                        teloxide::types::PublicChatKind::Supergroup(_) => "supergroup", 
                        teloxide::types::PublicChatKind::Channel(_) => "channel",
                    }
                }
            };
            
            let mut info = format!("👤 Forwarded from\n");
            info.push_str(&format!(" ├ chat: {}\n", title));
            info.push_str(&format!(" ├ type: {}\n", chat_type));
            info.push_str(&format!(" └ id: {}\n", chat.id));
            info
        }
    }
}

async fn send_telegram_message(chat_id: i64, text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let token = std::env::var("TELOXIDE_TOKEN")?;
    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
    
    let payload = json!({
        "chat_id": chat_id,
        "text": text,
        "parse_mode": "HTML"
    });
    
    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(&payload)
        .send()
        .await?;
    
    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(format!("Telegram API error: {}", error_text).into());
    }
    
    Ok(())
}

async fn process_message(message: &Message) -> Result<String, Box<dyn std::error::Error>> {
    let mut response = String::new();
    
    if let Some(user) = message.from() {
        response.push_str(&format_user_info(user, "You"));
    }
    
    response.push_str("\n");
    response.push_str(&format_chat_info(&message.chat));
    
    if let Some(forward_from) = message.forward_from() {
        response.push_str("\n");
        response.push_str(&format_forwarded_from_info(forward_from));
        
        response.push_str("\n📃 Message\n");
        if let Some(forward_date) = message.forward_date() {
            response.push_str(&format!(" └ forward_date: {}\n", forward_date.format("%a, %d %b %Y %H:%M:%S GMT")));
        }
    }
    
    Ok(response)
}

async fn process_start_command(message: &Message) -> Result<String, Box<dyn std::error::Error>> {
    let user = message.from().ok_or("No user in message")?;
    
    let mut welcome_text = format!("Hi {}!\n\n", &user.first_name);
    welcome_text.push_str("🤖 Telegram ID Bot\n\n");
    welcome_text.push_str("How this bot works:\n");
    welcome_text.push_str("• Send me any message to see your detailed user information\n");
    welcome_text.push_str("• Forward any message to me to see both your info and the original sender's details\n");
    welcome_text.push_str("• I can estimate account creation dates based on user IDs\n");
    welcome_text.push_str("• All information is displayed in a clean tree format\n\n");
    welcome_text.push_str("Try sending me a message or forwarding one to see it in action!");
    
    Ok(welcome_text)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    // Handle CORS preflight
    if req.method() == "OPTIONS" {
        return Ok(Response::builder()
            .status(200)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "POST, OPTIONS")
            .header("Access-Control-Allow-Headers", "Content-Type")
            .body("".into())?);
    }

    // Only handle POST requests for webhooks
    if req.method() != "POST" {
        return Ok(Response::builder()
            .status(405)
            .header("Access-Control-Allow-Origin", "*")
            .body("Method not allowed".into())?);
    }

    // Parse the incoming update
    let body = req.body();
    let update: Update = match serde_json::from_slice(body) {
        Ok(update) => update,
        Err(e) => {
            eprintln!("Failed to parse update: {}", e);
            return Ok(Response::builder()
                .status(400)
                .header("Access-Control-Allow-Origin", "*")
                .body("Invalid JSON".into())?);
        }
    };

    // Process the update
    match update.kind {
        UpdateKind::Message(message) => {
            let chat_id = message.chat.id.0;
            
            // Check if it's a command
            if let Some(text) = message.text() {
                if text.starts_with("/start") {
                    match process_start_command(&message).await {
                        Ok(response_text) => {
                            if let Err(e) = send_telegram_message(chat_id, &response_text).await {
                                eprintln!("Failed to send start message: {}", e);
                            }
                        }
                        Err(e) => eprintln!("Failed to process start command: {}", e),
                    }
                } else if text.starts_with("/help") {
                    let help_text = "Available commands:\n/start - Start the bot\n/help - Show this help message";
                    if let Err(e) = send_telegram_message(chat_id, help_text).await {
                        eprintln!("Failed to send help message: {}", e);
                    }
                } else {
                    // Regular message
                    match process_message(&message).await {
                        Ok(response_text) => {
                            if let Err(e) = send_telegram_message(chat_id, &response_text).await {
                                eprintln!("Failed to send message: {}", e);
                            }
                        }
                        Err(e) => eprintln!("Failed to process message: {}", e),
                    }
                }
            } else {
                // Non-text message (photo, document, etc.)
                match process_message(&message).await {
                    Ok(response_text) => {
                        if let Err(e) = send_telegram_message(chat_id, &response_text).await {
                            eprintln!("Failed to send message: {}", e);
                        }
                    }
                    Err(e) => eprintln!("Failed to process message: {}", e),
                }
            }
        }
        _ => {
            // Ignore other update types
            println!("Received non-message update: {:?}", update.kind);
        }
    }

    Ok(Response::builder()
        .status(200)
        .header("Access-Control-Allow-Origin", "*")
        .body("OK".into())?)
}