use crate::domains;
use crate::HouseDBConn;
use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::responders::to_named_measures;

#[derive(Serialize)]
pub struct JSONResponse<T> {
    pub value: T,
    pub error: &'static str,
}

pub type MeasureTypes = Vec<domains::measure_type::MeasureType>;

#[get("/measure_types")]
pub fn get_measure_types(conn: HouseDBConn) -> Json<JSONResponse<MeasureTypes>> {
    let result = domains::measure_type::select_measure_types(&*conn);

    match result {
        Ok(measures) => Json(JSONResponse {
            value: measures,
            error: "",
        }),
        _ => Json(JSONResponse {
            value: vec![],
            error: "There was an error :(",
        }),
    }
}

#[derive(Serialize)]
pub struct SplitMeasures {
    pub particles: Vec<f64>,
    pub co2: Vec<f64>,
    pub temp: Vec<f64>,
    pub humidity: Vec<f64>,
}
#[get("/measures?<start>&<end>")]
pub fn get_measures(
    conn: HouseDBConn,
    start: Option<String>,
    end: Option<String>,
) -> Json<JSONResponse<SplitMeasures>> {
    let measure_types = match domains::measure_type::select_measure_types(&*conn) {
        Ok(types) => types,
        _ => vec![],
    };

    match domains::measures::select_measures(&conn, start, end) {
        Ok(measures) => Json(JSONResponse {
            value: to_named_measures(measures, measure_types),
            error: "",
        }),
        _ => Json(JSONResponse {
            value: SplitMeasures {
                particles: vec![],
                co2: vec![],
                temp: vec![],
                humidity: vec![],
            },
            error: "There was an error :(",
        }),
    }
}

#[derive(Debug, Deserialize)]
pub struct MeasureRecord {
    pub pm02: u32,
    pub rco2: u32,
    pub atmp: f32,
    pub rhum: u32,
}

#[post("/measures", data = "<measure>")]
pub fn post_measures(conn: HouseDBConn, measure: Json<MeasureRecord>) -> Status {
    match domains::measures::insert_measurement(&conn, measure) {
        Ok(_) => Status::Created,
        _ => Status::InternalServerError,
    }
}
