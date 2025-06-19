use axum::extract::{Path, State};
use axum_extra::extract::Query;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::infra::{PoiRepository, dto::PoiDto};

use super::{ApiResult, AppState, Json, MessageResponse};

pub async fn get(
    State(state): State<AppState>,
    Query(query): Query<PoiFilter>,
) -> ApiResult<Vec<PoiDto>> {
    let points = state.db().get(query).await?;

    Ok(Json(points))
}

pub async fn put(
    State(state): State<AppState>,
    Path(id): Path<ObjectId>,
    Json(poi): Json<Poi>,
) -> ApiResult<MessageResponse> {
    state.db().put(id, poi).await?;

    Ok(MessageResponse::new("resource modified"))
}

pub async fn add(
    State(state): State<AppState>,
    Json(params): Json<Poi>,
) -> ApiResult<MessageResponse> {
    state.db().add(params).await?;

    Ok(MessageResponse::new("resource added"))
}

#[derive(Deserialize)]
pub struct PoiFilter {
    #[serde(default)]
    pub tags: Vec<String>,
    pub approved: Option<bool>,
}

#[derive(Deserialize, Serialize)]
pub struct ComercialPoi {
    pub name: String,
    pub description: String,
    pub images: Vec<String>,
    pub coords: Coordinates,
    pub tags: Vec<String>,

    pub instagram: String,
    #[serde(default)]
    pub approved: bool,
}

pub type Coordinates = (f64, f64);

#[derive(Deserialize, Serialize)]
pub struct TouristPoi {
    pub name: String,
    pub description: String,
    pub images: Vec<String>,
    pub coords: Coordinates,
    pub tags: Vec<String>,
    #[serde(default)]
    pub approved: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Poi {
    Comercial(ComercialPoi),
    Tourist(TouristPoi),
}

#[cfg(test)]
mod tests {
    use crate::app::poi::{ComercialPoi, Poi};

    #[test]
    fn test_add() {
        let a = Poi::Comercial(ComercialPoi {
            name: "Fast Lanches".to_string(),
            description: "Lanchonete muito legal".to_string(),
            images: vec!["".to_string()],
            coords: (-27.8187689345354, -50.33193942426937),
            tags: vec!["restaurante".to_string()],
            instagram: "@fast.lages".to_string(),
            approved: false,
        });

        let a = serde_json::to_string(&a).unwrap();

        panic!("{a}");
    }
}
