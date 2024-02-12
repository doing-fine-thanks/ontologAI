use futures::future::join_all;

use log::{debug, info};

use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};

use crate::templates::OntologicalRepresentationsTemplate;

const LOW_TOKEN_MODEL: &str = "gpt-3.5-turbo";

pub async fn create_ontological_representation(search_term: &String) -> Result<OntologicalRepresentationsTemplate, Box<dyn std::error::Error>> {
    
    let tones_prompt = String::from(
        "create a javascript script that is just a list of notes and their durations and start times (as floats) to be given the tonejs.vary the durations and start times dramatically. make ure the the notes dont just go up a scale. makes these notes represent a {}. name the variable 'melody'. make sure the script only constains the list.", 
    ).replace("{}", &search_term);
    let tones_future = send_query(tones_prompt);

    let dot_prompt = String::from(
        "output a complex, multilevel graph in dot graphViz notation representing the ontological nature and cultural impact of {}", 
    ).replace("{}", &search_term);
    let dot_graph_future = send_query(dot_prompt);

    let p5_prompt = String::from(
        "create p5js script instance that doesn't utilize the global setup function. make sure that the canvas childs in self to a div with the id 'p5canvas'. in the script create a model of {} only using 3D primatives via functions such as 'box', 'torus', 'freeGeometry', 'plane', 'sphere', 'cylinder', 'cone', 'ellipsoid' but dont use any functions like 'createGroup' or 'createBox'. wrap this in a function named 'sketch' where I can give some parameter p which will be a p5 instance for each of these functions to be assigned to.  Do not call the function even as an example. do not use the createGroup function."
    ).replace("{}", &search_term);
    let p5_spec_future = send_query(p5_prompt);


    let svg_prompt = String::from("create an html fragment of an svg of a {}").replace("{}", &search_term);
    let svg_future = send_query(svg_prompt);

    let complete_futures = join_all(vec![tones_future, dot_graph_future, p5_spec_future, svg_future]).await;

    let mut tones = complete_futures[0].as_ref().unwrap().clone();
    let mut dot_graph = complete_futures[1].as_ref().unwrap().clone();
    let mut p5_spec = complete_futures[2].as_ref().unwrap().clone();
    let mut svg_spec = complete_futures[3].as_ref().unwrap().clone();

    debug!("dot: \n{}", &dot_graph);
    debug!("tones: \n{}", &tones);
    debug!("p5: \n{}", &p5_spec);
    debug!("svg: \n{}", &svg_spec);

    if tones.contains("```") {
        tones = tones.replace("```javascript", "```")
        .replace("```js", "```")
        .split("```")
        .filter(|x| x.contains("];"))
        .collect::<Vec<&str>>()
        .get(0)
        .unwrap_or(&format!("Error parsing tone response: {}", &tones).as_str()).to_string();
    };

    if dot_graph.contains("```") {
        dot_graph = dot_graph.split("```")
        .collect::<Vec<&str>>()[1]
        .to_string()
        .replace("dot", "");
    }

    if p5_spec.contains("```") {
        p5_spec = p5_spec.replace("```javascript", "```")
        .replace("```js", "```")
        .split("```")
        .filter(|x| x.contains("function sketch"))
        .collect::<Vec<&str>>()
        .get(0)
        .unwrap_or(&format!("Error parsing model creation response: {}", &p5_spec).as_str()).to_string();
    };


    if svg_spec.contains("```") {
        svg_spec = svg_spec.replace("```html", "```")
        .split("```")
        .filter(|x| x.contains("</svg>"))
        .collect::<Vec<&str>>()
        .get(0)
        .unwrap_or(&format!("Error parsing svg response: {}", &svg_spec).as_str()).to_string();
    };



    let ontology = OntologicalRepresentationsTemplate {
        p5_spec: p5_spec,
        dot_graph_spec: dot_graph,
        serialized_tone_js_song: tones,
        voicable_description: String::from(""),
        svg_spec: svg_spec
    };

    Ok(ontology)

    
}

pub async fn send_query(prompt: String) -> Result<String, Box<dyn std::error::Error>> {
    info!("doing the chatGPT thing...");
    debug!("prompt is : \n {} \n\n", &prompt);

    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(1000 as u16)
        .model(LOW_TOKEN_MODEL)
        .messages([ChatCompletionRequestMessageArgs::default().role(Role::User).content(prompt).build()?])
        .build()?;

    let response = client.chat().create(request).await?;

    Ok(response.choices.first().unwrap().message.content.clone().unwrap())
}
