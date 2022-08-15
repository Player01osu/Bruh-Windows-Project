use actix_web::{HttpRequest, HttpResponse, Result};
use std::io::BufReader;
use std::path::PathBuf;
use std::{collections::HashMap, fs::File};

use actix_web::web::Bytes;
use anyhow::Ok;
use serde::{Deserialize, Serialize};

use ahash::RandomState;
use compact_str::*;
use dashmap::DashMap;
use once_cell::sync::*;

const STATIC_FILES: &str = "static.json";

#[derive(Deserialize, Serialize, Default)]
pub struct Static {
    routes: HashMap<String, PathBuf>,
    error_pages: HashMap<String, PathBuf>,
}

#[derive(Debug)]
pub struct RouteHandle {
    pub response: StaticFile,
}

#[derive(Debug)]
pub struct StaticFile {
    pub bytes: Bytes,
    pub path: PathBuf,
}

pub static ROUTEMAP: Lazy<DashMap<CompactString, RouteHandle, RandomState>> =
    Lazy::new(|| DashMap::with_hasher(RandomState::new()));

pub async fn router(req: HttpRequest) -> Result<HttpResponse> {
    match ROUTEMAP.get(req.path()) {
        Some(handler) => {
            let handler = handler.value();

            core::result::Result::Ok(
                HttpResponse::Ok().body(handler.response.bytes.to_owned()),
            )
        }
        None => {
            let not_found_handler = ROUTEMAP
                .get("{{404}}")
                .expect("Could not retrieve 404 files");
            let handle = not_found_handler.value();
            let not_found_response = &handle.response.bytes;

            core::result::Result::Ok(
                HttpResponse::NotFound().body(not_found_response.to_owned()),
            )
        }
    }
}

impl StaticFile {
    fn create(path: &PathBuf) -> anyhow::Result<Self> {
        let files_to_bytes = Bytes::from(std::fs::read(path)?);

        Ok(Self {
            bytes: files_to_bytes,
            path: path.to_path_buf(),
        })
    }
}

pub fn init() -> anyhow::Result<()> {
    let static_file = File::open(STATIC_FILES)?;
    let buf_reader = BufReader::new(static_file);
    let static_parsed: Static = serde_json::from_reader(buf_reader)?;

    RouteHandle::add_routes(&static_parsed.routes)?;

    crate::run()?;
    anyhow::Ok(())
}

impl RouteHandle {
    fn add_routes(routing_list: &HashMap<String, PathBuf>) -> anyhow::Result<()> {
        for (route, path) in routing_list {
            let route_handle = RouteHandle {
                response: StaticFile::create(path)?,
            };

            let mut route_fmt = route.chars();
            let mut route_str: String = route.as_str().into();

            while route_str.starts_with('/') {
                route_fmt.next();
                route_str = route_fmt.as_str().into();
            }

            route_str = format!("/{}", route_str);

            ROUTEMAP.insert(route_str.into(), route_handle);
        }

        Ok(())
    }
}
