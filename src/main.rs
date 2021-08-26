use anyhow::Result;
use ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport;
use ic_agent::identity::AnonymousIdentity;
use ic_agent::{Agent, Identity};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let identity = MyCoolIdentity::create();
    let arc_identity = Arc::from(identity);
    // Assume replica is running locally with following url
    let transport = ReqwestHttpReplicaV2Transport::create("http://127.0.0.1:8000")?;
    let agent = Agent::builder()
        .with_arc_identity(arc_identity.clone())
        .with_transport(transport)
        .build()?;
    agent.fetch_root_key().await?;
    let status = agent.status().await?;
    eprintln!("{:?}", status);

    // still have full access to the identity implementation
    arc_identity.cool_function();
    Ok(())
}

pub struct MyCoolIdentity {
    pub anonymous: Box<AnonymousIdentity>,
}

impl MyCoolIdentity {
    pub fn create() -> Self {
        Self {
            anonymous: Box::new(AnonymousIdentity),
        }
    }

    pub fn cool_function(&self) {
        println!("Calling cool_function() of MyCoolIdentity!");
    }
}

impl Identity for MyCoolIdentity {
    fn sender(&self) -> Result<ic_agent::ic_types::Principal, String> {
        eprintln!("Calling sender() of MyCoolIdentity!");
        self.anonymous.sender()
    }

    fn sign(&self, blob: &[u8]) -> Result<ic_agent::Signature, String> {
        eprintln!("Calling sign() of MyCoolIdentity!");
        self.anonymous.sign(blob)
    }
}
