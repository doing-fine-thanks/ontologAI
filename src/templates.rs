use askama_actix::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "ontology_display.html")]
pub struct OntologicalRepresentationsTemplate {
    pub p5_spec: String,
    pub dot_graph_spec: String,
    pub serialized_tone_js_song: String,
    pub voicable_description: String,
}
