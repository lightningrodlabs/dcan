use hdk::prelude::*;
use coordinator_integrity::*;

// #[derive(Serialize, Deserialize, Debug)]
// pub struct AddSpamReporterForCoordinationInput {
//     pub base_coordination_hash: ActionHash,
//     pub target_spam_reporter: AgentPubKey,
// }
#[hdk_extern]
pub fn add_spam_reporter_for_coordination(coordination_hash: ActionHash) -> ExternResult<()> {
    let spam_reporter: AgentPubKey = agent_info()?.agent_latest_pubkey.into();
    create_link(coordination_hash.clone(), spam_reporter.clone(), LinkTypes::CoordinationToSpamReporters, ())?;
    create_link(spam_reporter, coordination_hash, LinkTypes::SpamReporterToCoordinations, ())?;

    Ok(())
}

#[hdk_extern]
pub fn get_spam_reporters_for_coordination(coordination_hash: ActionHash) -> ExternResult<Vec<AgentPubKey>> {
    let links = get_links(coordination_hash, LinkTypes::CoordinationToSpamReporters, None)?;
    
    let agents: Vec<AgentPubKey> = links
        .into_iter()
        .map(|link| AgentPubKey::from(EntryHash::from(link.target)))
        .collect();
    let mut unduped_agents = vec![];
    for agent in agents {
        if !unduped_agents.contains(&agent) {
            unduped_agents.push(agent);
        }
    }
    Ok(unduped_agents)
}


#[hdk_extern]
pub fn get_coordinations_for_spam_reporter(spam_reporter: AgentPubKey) -> ExternResult<Vec<Record>> {
    let links = get_links(spam_reporter, LinkTypes::SpamReporterToCoordinations, None)?;
    
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(ActionHash::from(link.target).into(), GetOptions::default()))
        .collect();

    // Get the records to filter out the deleted ones
    let records: Vec<Record> = HDK.with(|hdk| hdk.borrow().get(get_input))?
        .into_iter()
        .filter_map(|r| r)
        .collect();

    Ok(records)
}
        
// #[derive(Serialize, Deserialize, Debug)]
// pub struct RemoveSpamReporterForCoordinationInput {
//     pub base_coordination_hash: ActionHash,
//     pub target_spam_reporter: AgentPubKey,
// }
#[hdk_extern]
pub fn remove_spam_reporter_for_coordination(coordination_hash: ActionHash ) -> ExternResult<()> {
    let spam_reporter: AgentPubKey = agent_info()?.agent_latest_pubkey.into();

    let links = get_links(coordination_hash.clone(), LinkTypes::CoordinationToSpamReporters, None)?;
    
    for link in links {
        if AgentPubKey::from(EntryHash::from(link.target.clone())).eq(&spam_reporter) {
            delete_link(link.create_link_hash)?;
        }
    }
    
    let links = get_links(spam_reporter.clone(), LinkTypes::SpamReporterToCoordinations, None)?;

    for link in links {
        if ActionHash::from(link.target.clone()).eq(&coordination_hash) {
            delete_link(link.create_link_hash)?;
        }
    }

    Ok(())        
}