use anyhow::Error;
use image::{DynamicImage, GenericImageView};
use rand::seq::SliceRandom;

use crate::{
    cluster::{Cluster, PixelColor},
    EuclideanDistance,
};

pub fn k_cluster(image: &DynamicImage, k: usize) -> Result<Vec<Cluster>, Error> {
    let (points, mut clusters) = sample_data(image, k);

    points.iter().for_each(|p| {
        let (_, cluster) = clusters
            .iter_mut()
            .map(|c| (c.center.euclidean_dist(p), c))
            .min_by(|c1, c2| c1.0.partial_cmp(&c2.0).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap();

        cluster.points.push(*p);
    });

    clusters.iter_mut().for_each(|c| c.update_centroid());

    Ok(clusters)
}

fn sample_data(image: &DynamicImage, k: usize) -> (Vec<PixelColor>, Vec<Cluster>) {
    let points: Vec<_> = image.pixels().map(|p| p.2).collect();

    let clusters: Vec<_> = points
        .choose_multiple(&mut rand::thread_rng(), k)
        .map(|color| Cluster::new(*color))
        .collect();

    (points, clusters)
}
