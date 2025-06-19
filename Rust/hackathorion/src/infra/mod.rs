pub mod dto;

use anyhow::Error;
use dto::{ComercialPoiDto, PoiDto, RouteDto, RouteVm, TouristPoiDto};
use futures::stream::TryStreamExt;
use mongodb::{
    Database,
    bson::{doc, oid::ObjectId},
};
use urlencoding::encode;

use crate::app::{
    poi::{Poi, PoiFilter},
    route::{Waypoint, export_to_maps_url},
};

type DbResult<T> = Result<T, Error>;

pub trait PoiRepository {
    async fn add(&self, poi: Poi) -> DbResult<()>;
    async fn get(&self, tags: PoiFilter) -> DbResult<Vec<PoiDto>>;
    async fn get_by_id(&self, id: ObjectId) -> DbResult<Option<PoiDto>>;
    async fn put(&self, id: ObjectId, poi: Poi) -> DbResult<()>;
}

pub trait RouteRepository {
    async fn get(&self) -> DbResult<Vec<RouteVm>>;
}

impl RouteRepository for Database {
    async fn get(&self) -> DbResult<Vec<RouteVm>> {
        let routes = self.collection("route").find(doc! {}).await?;

        let routes: Vec<RouteDto> = routes.try_collect().await?;

        let mut output = vec![];

        for route in routes {
            let mut waypoints = vec![];

            for w in route.waypoints {
                let waypoint = self.get_by_id(w).await?.unwrap();
                waypoints.push(waypoint);
            }

            let points: Vec<_> = waypoints
                .iter()
                .map(|w| Waypoint {
                    lat: w.coord().1,
                    lon: w.coord().0,
                })
                .collect();

            output.push(RouteVm {
                waypoints,
                id: route.id,
                image: route.image,
                description: route.description,
                title: route.title,
                google_maps_route: export_to_maps_url(&points),
            });
        }

        Ok(output)
    }
}

pub fn export_maps_location(point: &Waypoint) -> String {
    let query = format!("{},{}", point.lat, point.lon);
    let encoded_query = encode(&query).to_string();

    format!(
        "https://www.google.com/maps/search/?api=1&query={}",
        encoded_query
    )
}

impl PoiRepository for Database {
    async fn add(&self, poi: Poi) -> DbResult<()> {
        let poi = match poi {
            Poi::Comercial(poi) => PoiDto::Comercial(ComercialPoiDto {
                approved: poi.approved,
                coords: poi.coords,
                description: poi.description,
                google_maps_route: export_maps_location(&Waypoint {
                    lat: poi.coords.0,
                    lon: poi.coords.1,
                }),
                id: ObjectId::new(),
                images: poi.images,
                instagram: poi.instagram,
                tags: poi.tags,
                name: poi.name,
            }),
            Poi::Tourist(poi) => PoiDto::Tourist(TouristPoiDto {
                approved: poi.approved,
                coords: poi.coords,
                description: poi.description,
                google_maps_route: export_maps_location(&Waypoint {
                    lat: poi.coords.0,
                    lon: poi.coords.1,
                }),
                id: ObjectId::new(),
                images: poi.images,
                tags: poi.tags,
                name: poi.name,
            }),
        };

        self.collection("poi").insert_one(poi).await?;

        Ok(())
    }

    async fn get(&self, filter: PoiFilter) -> DbResult<Vec<PoiDto>> {
        let mut doc_filter = doc! {};

        if let Some(approved) = filter.approved {
            doc_filter.insert("approved", approved);
        }

        if !filter.tags.is_empty() {
            doc_filter.insert("tags", doc! { "$in": filter.tags });
        }

        let cursor = self.collection("poi").find(doc_filter).await?;

        let results = cursor.try_collect().await?;

        Ok(results)
    }

    async fn put(&self, id: ObjectId, poi: Poi) -> DbResult<()> {
        let filter = doc! { "_id": id };

        self.collection("poi").replace_one(filter, poi).await?;

        Ok(())
    }

    async fn get_by_id(&self, id: ObjectId) -> DbResult<Option<PoiDto>> {
        let filter = doc! { "_id": id };

        let result = self.collection("poi").find_one(filter).await?;

        Ok(result)
    }
}
