use owo_colors::OwoColorize;
use simple_websockets::{Event, Responder};
use std::collections::HashMap;
use std::sync::mpsc;
use std::time::Duration;
use md5;
use chrono::Utc;
use dotenv::dotenv;

#[tokio::main]
async fn main() {

    dotenv().ok();

    let port = std::env::var("PORT").expect("PORT must be set.");

    println!("{}", "Fortnite matchmaker by Zetax".purple().bold());
    let version = env!("CARGO_PKG_VERSION");
    println!("Version: {}", version);

    let event_hub = simple_websockets::launch(port.parse().unwrap())
        .expect("failed to listen on port 8080");

    println!("Listening on port {}", port);

    let (message_sender, _message_receiver) = mpsc::channel::<(u64, simple_websockets::Message)>();
    let mut clients: HashMap<u64, Responder> = HashMap::new();

    loop {
        let event = event_hub.poll_event();

        match event {
            Event::Connect(client_id, responder) => {
                let ticket_id = calculate_id();
                let match_id = calculate_id();
                let session_id = calculate_id();

                println!("A client connected with id #{}", client_id);
                let json = r#"
                    {
                        "payload": {
                            "state": "Connecting"
                        },
                        "name": "StatusUpdate"
                    }
                "#;

                let message_to_send = simple_websockets::Message::Text(json.to_string());
                let _ = responder.send(message_to_send);

                tokio::time::sleep(Duration::from_millis(200)).await;

                let message2 = format!(
                    r#"
                {{
                    "payload": {{
                    "totalPlayers": 0,
                    "connectedPlayers": 0,
                    "state": "waiting"
                    }},
                    "name": "StatusUpdate"
                }}
                "#, );

                let _ = responder.send(simple_websockets::Message::Text(message2));

                tokio::time::sleep(Duration::from_secs(1)).await;

                let message3 = format!(
                    r#"
                {{
                    "payload": {{
                        "ticketId": "{}",
                        "queuedPlayers": 0,
                        "estimatedWaitSec": 0,
                        "status": {{}},
                        "state": "Queued"
                    }},
                    "name": "StatusUpdate"
                }}
                "#,
                    ticket_id
                );

                let _ = responder.send(simple_websockets::Message::Text(message3));

                tokio::time::sleep(Duration::from_secs(1)).await;

                let message4 = format!(
                    r#"
                {{
                    "payload": {{
                        "matchId": "{}",
                        "state": "SessionAssignment"
                    }},
                    "name": "StatusUpdate"
                }}
                "#,
                    match_id
                );

                let _ = responder.send(simple_websockets::Message::Text(message4));

                tokio::time::sleep(Duration::from_secs(2)).await;

                let message5 = format!(
                    r#"
                {{
                    "payload": {{
                        "matchId": "{}",
                        "sessionId": "{}",
                        "joinDelaySec": 1
                    }},
                    "name": "Play"
                }}
                "#,
                    match_id, session_id
                );

                let _ = responder.send(simple_websockets::Message::Text(message5));

                clients.insert(client_id, responder);

                tokio::time::sleep(Duration::from_secs(5)).await;

                Responder::close(&clients[&client_id]);
            },
            Event::Disconnect(client_id) => {
                clients.remove(&client_id);
            },
            Event::Message(client_id, message) => {
                let _ = message_sender.send((client_id, message));
            },
        }

    }
}

fn calculate_id() -> String {
    let timestamp = Utc::now().timestamp_millis();
    let input = format!("1{}", timestamp);
    let hash = md5::compute(input);
    format!("{:x}", hash)
}
