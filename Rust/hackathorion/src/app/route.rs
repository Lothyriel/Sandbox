use axum::extract::State;
use polyline::decode_polyline;
use serde_json::json;
use urlencoding::encode;

use crate::infra::{RouteRepository, dto::RouteVm};

use super::{ApiResult, AppState, Json, poi::Coordinates};

pub async fn calculate(
    State(state): State<AppState>,
    Json(params): Json<RouteParams>,
) -> ApiResult<RouteInfo> {
    let res = openroute_calculate(state, params).await?;

    let res = extract_waypoint_coordinates(res)?;

    Ok(Json(res))
}

pub async fn suggested(State(state): State<AppState>) -> ApiResult<Vec<RouteVm>> {
    let routes = state.db().get().await?;

    Ok(Json(routes))
}

fn extract_waypoint_coordinates(response: ORSResponse) -> Result<RouteInfo, anyhow::Error> {
    let route = response
        .routes
        .first()
        .ok_or(anyhow::anyhow!("Invalid route"))?;

    let decoded = decode_polyline(&route.geometry, 5)?;

    let waypoints: Vec<Waypoint> = route
        .way_points
        .iter()
        .filter_map(|&index| decoded.0.get(index))
        .map(|coord| Waypoint {
            lat: coord.x,
            lon: coord.y,
        })
        .collect();

    Ok(RouteInfo {
        distance: route.summary.distance,
        duration: route.summary.duration,
        google_maps_route: export_to_maps_url(&waypoints),
        waypoints,
    })
}

#[derive(Debug, Deserialize)]
struct ORSResponse {
    routes: Vec<Route>,
}

#[derive(Debug, Deserialize)]
struct Route {
    geometry: String,
    way_points: Vec<usize>,
    summary: Summary,
}

#[derive(Debug, Deserialize)]
pub struct Summary {
    distance: f64,
    duration: f64,
}

#[derive(Debug, Serialize, Clone, Copy)]
pub struct Waypoint {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Serialize)]
pub struct RouteInfo {
    distance: f64,
    duration: f64,
    waypoints: Vec<Waypoint>,
    google_maps_route: String,
}

async fn openroute_calculate(
    state: AppState,
    params: RouteParams,
) -> Result<ORSResponse, anyhow::Error> {
    let coordinates: Vec<[f64; 2]> = params.waypoints.iter().map(|p| [p.1, p.0]).collect();

    let body = json!({
        "coordinates": coordinates,
        "radiuses": vec![-1.0; coordinates.len()]
    });

    tracing::warn!("body: {}", body);

    let res = state
        .http_client
        .post("https://api.openrouteservice.org/v2/directions/driving-car/json")
        .header("Authorization", state.openroute_key)
        .json(&body)
        .send()
        .await?;

    let response = res.json().await?;

    Ok(response)
}

pub fn export_to_maps_url(waypoints: &[Waypoint]) -> String {
    let mode = "3e0";

    let encoded_waypoints: Vec<String> = waypoints
        .iter()
        .map(|point| encode(&format!("{}, {}", point.lon, point.lat)).to_string())
        .collect();

    let url = format!(
        "https://www.google.com/maps/dir/{}/data=!{}",
        encoded_waypoints.join("/"),
        mode,
    );

    url
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RouteResponse {
    routes: Vec<CandidateRoute>,
}

#[derive(Deserialize)]
pub struct CandidateRoute {
    summary: Summary,
    geometry: String,
}

#[derive(Deserialize, Serialize)]
pub struct RouteParams {
    pub waypoints: Vec<Coordinates>,
}

#[derive(Serialize)]
pub struct RouteWaypoints {
    waypoints: Vec<Coordinates>,
}
