use mongodb::bson::oid::ObjectId;
use mongodb::bson::serde_helpers::serialize_object_id_as_hex_string;
use serde::{Deserialize, Serialize};

use crate::app::poi::Coordinates;

#[derive(Deserialize, Serialize)]
pub struct ComercialPoiDto {
    #[serde(rename = "_id")]
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub name: String,
    pub description: String,
    pub images: Vec<String>,
    pub coords: Coordinates,
    pub tags: Vec<String>,
    pub instagram: String,
    pub approved: bool,
    pub google_maps_route: String,
}

#[derive(Deserialize, Serialize)]
pub struct TouristPoiDto {
    #[serde(rename = "_id")]
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub name: String,
    pub description: String,
    pub images: Vec<String>,
    pub coords: Coordinates,
    pub tags: Vec<String>,
    pub approved: bool,
    pub google_maps_route: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PoiDto {
    Comercial(ComercialPoiDto),
    Tourist(TouristPoiDto),
}

impl PoiDto {
    pub fn coord(&self) -> Coordinates {
        match self {
            PoiDto::Comercial(poi) => poi.coords,
            PoiDto::Tourist(poi) => poi.coords,
        }
    }
}

#[derive(Serialize)]
pub struct RouteVm {
    #[serde(rename = "_id")]
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub waypoints: Vec<PoiDto>,
    pub image: String,
    pub title: String,
    pub description: String,
    pub google_maps_route: String,
}

#[derive(Deserialize)]
pub struct RouteDto {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub waypoints: Vec<ObjectId>,
    pub image: String,
    pub description: String,
    pub title: String,
}
