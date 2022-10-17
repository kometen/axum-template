use std::net::SocketAddr;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    extract::{Json, rejection::JsonRejection}, Router
};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Jeg æder blåbærsyltetøj!"
}

async fn create_user(payload: Result<Json<CreateUser>, JsonRejection>) -> impl IntoResponse {
    match payload {
        Ok(payload) => {
            let uuid = Uuid::new_v4();
            let user = User {
                id: uuid,
                username: payload.username.clone(),
            };

            let status_message = StatusMessage {
                id: 1,
                description: user.username + " created"
            };

            (StatusCode::CREATED, Json(status_message))
        }

        Err(JsonRejection::MissingJsonContentType(_)) => {
            let status_message = StatusMessage {
                id: 86,
                description: "Request didn't have `Content-Type: application/json` header".to_string()
            };
            // Request didn't have `Content-Type: application/json` header

            (StatusCode::BAD_REQUEST, Json(status_message))
        }

        Err(JsonRejection::JsonDataError(_)) => {
            let status_message = StatusMessage {
                id: 87,
                description: "Couldn't deserialize the body into the target type".to_string()
            };
            // Couldn't deserialize the body into the target type

            (StatusCode::BAD_REQUEST, Json(status_message))
        }

        Err(JsonRejection::JsonSyntaxError(_)) => {
            let status_message = StatusMessage {
                id: 88,
                description: "Syntax error in the body".to_string()
            };
            // Syntax error in the body

            (StatusCode::BAD_REQUEST, Json(status_message))
        }

        Err(JsonRejection::BytesRejection(_)) => {
            let status_message = StatusMessage {
                id: 89,
                description: "Failed to extract the request body".to_string()
            };
            // Failed to extract the request body

            (StatusCode::BAD_REQUEST, Json(status_message))
        }

        Err(_) => {
            let status_message = StatusMessage {
                id: 99,
                description: "Unknown error".to_string()
            };

            (StatusCode::BAD_REQUEST, Json(status_message))
        }
    }
}

#[derive(Deserialize)]
struct CreateUser {
    username: String
}

#[derive(Serialize)]
struct User {
    id: Uuid,
    username: String
}

#[derive(Serialize)]
struct StatusMessage {
    id: u64,
    description: String
}
