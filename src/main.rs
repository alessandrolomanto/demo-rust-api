use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

// ============================================================================
// Data Models
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateItemRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

// ============================================================================
// Application State
// ============================================================================

type ItemStore = Arc<RwLock<HashMap<Uuid, Item>>>;

#[derive(Clone)]
pub struct AppState {
    items: ItemStore,
}

impl AppState {
    fn new() -> Self {
        Self {
            items: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

// ============================================================================
// Handlers
// ============================================================================

/// Health check endpoint
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: chrono::Utc::now(),
    })
}

/// List all items
async fn list_items(State(state): State<AppState>) -> Json<ApiResponse<Vec<Item>>> {
    let items = state.items.read().unwrap();
    let items_vec: Vec<Item> = items.values().cloned().collect();
    
    Json(ApiResponse {
        success: true,
        data: Some(items_vec),
        message: None,
    })
}

/// Get a single item by ID
async fn get_item(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Item>>, (StatusCode, Json<ApiResponse<()>>)> {
    let items = state.items.read().unwrap();
    
    match items.get(&id) {
        Some(item) => Ok(Json(ApiResponse {
            success: true,
            data: Some(item.clone()),
            message: None,
        })),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                data: None,
                message: Some(format!("Item with id {} not found", id)),
            }),
        )),
    }
}

/// Create a new item
async fn create_item(
    State(state): State<AppState>,
    Json(payload): Json<CreateItemRequest>,
) -> (StatusCode, Json<ApiResponse<Item>>) {
    let now = chrono::Utc::now();
    let item = Item {
        id: Uuid::new_v4(),
        name: payload.name,
        description: payload.description,
        created_at: now,
        updated_at: now,
    };

    let mut items = state.items.write().unwrap();
    items.insert(item.id, item.clone());

    (
        StatusCode::CREATED,
        Json(ApiResponse {
            success: true,
            data: Some(item),
            message: Some("Item created successfully".to_string()),
        }),
    )
}

/// Update an existing item
async fn update_item(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateItemRequest>,
) -> Result<Json<ApiResponse<Item>>, (StatusCode, Json<ApiResponse<()>>)> {
    let mut items = state.items.write().unwrap();

    match items.get_mut(&id) {
        Some(item) => {
            if let Some(name) = payload.name {
                item.name = name;
            }
            if let Some(description) = payload.description {
                item.description = Some(description);
            }
            item.updated_at = chrono::Utc::now();

            Ok(Json(ApiResponse {
                success: true,
                data: Some(item.clone()),
                message: Some("Item updated successfully".to_string()),
            }))
        }
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                data: None,
                message: Some(format!("Item with id {} not found", id)),
            }),
        )),
    }
}

/// Delete an item
async fn delete_item(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    let mut items = state.items.write().unwrap();

    match items.remove(&id) {
        Some(_) => Ok(Json(ApiResponse {
            success: true,
            data: None,
            message: Some("Item deleted successfully".to_string()),
        })),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse {
                success: false,
                data: None,
                message: Some(format!("Item with id {} not found", id)),
            }),
        )),
    }
}

// ============================================================================
// Router Setup
// ============================================================================

fn create_router(state: AppState) -> Router {
    Router::new()
        // Health check
        .route("/health", get(health))
        // Items CRUD
        .route("/api/v1/items", get(list_items).post(create_item))
        .route(
            "/api/v1/items/{id}",
            get(get_item).put(update_item).delete(delete_item),
        )
        // Middleware
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

// ============================================================================
// Main Entry Point
// ============================================================================

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "demo_rust_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create application state
    let state = AppState::new();

    // Build router
    let app = create_router(state);

    // Start server
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3030".to_string())
        .parse()
        .expect("PORT must be a valid u16");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("🚀 Demo Rust API listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

