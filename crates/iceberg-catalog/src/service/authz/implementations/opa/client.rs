use serde::{Deserialize, Serialize};

use crate::{
    service::{
        authn::Actor,
        authz::Result
    }
};


#[derive(Clone, Debug)]
pub struct Client {
    client: reqwest::Client,
    url: url::Url,
}

#[derive(Serialize, Deserialize, Debug)]
struct DataRequest {
    input: OPAInput,
}

#[derive(Serialize, Deserialize, Debug)]
struct DataResponse {
    result: Option<serde_json::Value>,
    decision_id: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct OPAInput {
    pub actor: OPAActor,
    // action: Action,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "kind", rename_all="camelCase")]
pub enum OPAActor {
    Anonymous,
    Principal { name: String },
}

impl From<&Actor> for OPAActor {
    fn from(actor: &Actor) -> Self {
        match actor {
            Actor::Anonymous => Self::Anonymous,
            Actor::Principal(principal) => Self::Principal { name: principal.to_string() },
            Actor::Role { principal: _, assumed_role: _ } => todo!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Action {

}

impl Client {
    pub fn new() -> Self {
        return Self {
            client: reqwest::Client::new(),
            url: url::Url::parse("http://localhost:8182/v1/data/lakekeeper/allowed").unwrap(),
        }
    }

    pub async fn check(&self, input: OPAInput) -> Result<bool> {
        let request = DataRequest { input };

        match self.client.post(self.url.clone())
            .json(&request)
            .send()
            .await {
                Ok(response) => {
                    let data_response = response.json::<DataResponse>().await.unwrap();

                    println!("res={:?}", data_response);
                    match data_response.result.unwrap() {
                        serde_json::Value::Bool(v) => Ok(v),
                        _ => todo!()
                    }
                }
                Err(err) => todo!(),
            }
    }
}
