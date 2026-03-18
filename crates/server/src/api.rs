use axum::{Router, routing::get, Json};
use scada_core::Tag;
use scada_core::RegisterValue;
use simulator::PlcSimulator;

async fn get_tags() -> Json<Vec<Tag>> {

    let sim = PlcSimulator::new(1);

    let mut tags = Vec::new();
    let mut tag1 = Tag::new(1, "Température", 40001);
    let mut tag2 = Tag::new(2, "Pression_1", 40002);
    let mut tag3 = Tag::new(3, "Pression_2", 40003);

    let valeur = sim.read_register(40001).unwrap_or(0);
    let valeur2 = sim.read_register(40002).unwrap_or(0);
    let valeur3 = sim.read_register(40003).unwrap_or(0);

    tag1.value = Some(RegisterValue::UInt16(valeur));
    tag2.value = Some(RegisterValue::UInt16(valeur2));
    tag3.value = Some(RegisterValue::UInt16(valeur3));

    tags.push(tag1);
    tags.push(tag2);
    tags.push(tag3);
    Json(tags)
}
pub fn router() -> Router {
    Router::new()
        .route("/tags", get(get_tags))
}