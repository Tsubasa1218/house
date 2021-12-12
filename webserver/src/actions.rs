use crate::domains;
use crate::HouseDBConn;
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

//#[get("/measures?<start>&<end>")]
//pub fn measures(conn: HouseDBConn, start_date: Option<String>, end_date: Option<String>) {
//}

#[derive(Debug, Deserialize)]
pub struct MeasureRecord<'a> {
    pub wifi: &'a str,
    pub chip: &'a str,
    pub pm02: u32,
    pub rco2: u32,
    pub atmp: f32,
    pub rhum: u32,
}

#[post("/measures", data = "<measure>")]
pub fn post_measures(conn: HouseDBConn, measure: Json<MeasureRecord>) {
    domains::measures::insert_measurement(&conn, measure);
}
