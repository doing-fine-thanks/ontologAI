use std::fmt::format;

use log::{debug, info};

use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};

use crate::templates::OntologicalRepresentationsTemplate;

const LOW_TOKEN_MODEL: &str = "gpt-3.5-turbo";

pub async fn create_ontological_representation(search_term: &String) -> Result<OntologicalRepresentationsTemplate, Box<dyn std::error::Error>> {
    let mut tones = send_query(
        &format!(
            "create a javascript script that is just a list of notes and their durations and start times (as floats) to be given the tonejs.vary the durations and start times dramatically. make ure the the notes dont just go up a scale. makes these notes represent a {}. name the variable 'melody'. make sure the script only constains the list.", 
            &search_term
        )
    ).await?;

    info!("{}", &tones);

    let mut dot_graph = send_query(
        &format!(
            "output a complex, multilevel graph in dot graph viz notation representing the ontological nature and cultural impact of {}", 
            &search_term
        )
    ).await?;

    let p5_spec = send_query(
        &format!(
            "create p5js script instance that doesn't utilize the global setup function. make sure that the canvas childs in self to a div with the id 'p5canvas'. in the script create a model of {} only using 3D primatives via functions like 'box' or 'torus'. wrap this in a function named 'sketch' where I can give some parameter p which will be a p5 instance for each of these functions to be assigned to.  Do not call the function even as an example. do not use the createGroup function.",
             &search_term,
        )
    ).await?;

    let voice_script = String::from("");
    //let voice_script = send_query(
    //    &format!(
    //        "create a one paragraph passage describing the ontology and gestalt of a {}", 
    //        &search_term
    //    )
    //).await?;

    if tones.contains("```") {
        tones = tones.replace("```javascript", "```").replace("```js", "```").split("```").collect::<Vec<&str>>()[1].to_string()
    }

    if dot_graph.contains("```") {
        dot_graph = dot_graph.split("```").collect::<Vec<&str>>()[1].to_string().replace("dot", "");
    }

    let ontology = OntologicalRepresentationsTemplate {
        p5_spec: p5_spec.replace("```javascript", "```").replace("```js", "```").split("```").collect::<Vec<&str>>()[1].to_string(),
        dot_graph_spec: dot_graph,
        serialized_tone_js_song: tones,
        voicable_description: voice_script
    };

    Ok(ontology)

    
}

pub async fn send_query(prompt: &String) -> Result<String, Box<dyn std::error::Error>> {
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
