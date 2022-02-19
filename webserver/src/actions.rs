use crate::domains;
use crate::HouseDBConn;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[derive(Serialize)]
pub struct MeasureTypesResponse {
    pub value: Vec<domains::measure_type::MeasureType>,
    pub error: &'static str,
}

#[get("/measure_types")]
pub fn get_measure_types(conn: HouseDBConn) -> Json<MeasureTypesResponse> {
    let result = domains::measure_type::select_measure_types(&*conn);
    match result {
        Ok(measures) => Json(MeasureTypesResponse {
            value: measures,
            error: "",
        }),
        _ => Json(MeasureTypesResponse {
            value: vec![],
            error: "There was an error :(",
        }),
    }
}

#[derive(Serialize)]
pub struct MeasuresResponse {
    pub value: Vec<domains::measures::MeasureValue>,
    pub error: &'static str,
}
#[get("/measures?<start>&<end>")]
pub fn get_measures(
    conn: HouseDBConn,
    start: Option<String>,
    end: Option<String>,
) -> Json<MeasuresResponse> {
    match domains::measures::select_measures(&conn, start, end) {
        Ok(measures) => Json(MeasuresResponse {
            value: measures,
            error: "",
        }),
        _ => Json(MeasuresResponse {
            value: vec![],
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
