use crate::domains;
use crate::HouseDBConn;
use rocket_contrib::json::Json;

#[derive(Serialize)]
pub struct MeasureTypesResponse {
    pub value: Vec<domains::measure_type::MeasureType>,
    pub error: &'static str,
}

#[get("/measure_types")]
pub fn measure_types(conn: HouseDBConn) -> Json<MeasureTypesResponse> {
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

//#[get("/measures")]
//pub fn measures(conn: HouseDBConn) {
//}
//
//
//#[post("/measure")]
//pub fn measures(conn: HouseDBConn) {
//}
