use crate::master_types::*;
use kompact::prelude::*;

#[derive(Debug, Clone)]
pub enum WorkerMessages {
    External(External),
    InternalStateUpdate(StateUpdate),
}
//NOTE/TODO: not sure I like all these nested enums for future match statements, will probably
//change later
impl From<MasterMessage> for WorkerMessages {
    fn from(item: MasterMessage) -> Self {
        match item {
            MasterMessage::Rfp => WorkerMessages::External(External::MasterMessage(item)),
            MasterMessage::AcceptedProposalBroadcast {
                seq_number,
                message,
            } => WorkerMessages::External(External::MasterMessage(item)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum External {
    WorkerRfpResponse,
    WorkerStateUpdateConfirmation,
    MasterMessage(MasterMessage),
}

#[derive(Debug, Clone)]
pub enum WorkerResponse {
    RfpResponse(RfpResponse),
    StateUpdateConfirmed(StateUpdate),
    // NOTE: acknowledgement mechanism as response to AcceptedProposalBroadcast from master
    // based on logic, master can then shutdown workers or send next rfp iteration when
    // received confirmations = num_workers
}

#[derive(Debug, Clone)]
pub struct RfpResponse {
    proposed_sequence_number: Option<i32>,
    msg: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct StateUpdate {
    seq_number: i32,
    message: i32,
}

#[derive(ComponentDefinition)]
pub struct Worker {
    ctx: ComponentContext<Self>,
    state: (i32, i32),
    proposed_sequence_number: Option<i32>,
    message_port: ProvidedPort<MessagePort>,
}
ignore_lifecycle!(Worker);

impl Worker {
    pub fn new() -> Self {
        Self {
            ctx: ComponentContext::uninitialised(),
            state: (0, 0),
            proposed_sequence_number: None,
            message_port: ProvidedPort::uninitialised(),
        }
    }
    fn update_state(&mut self, seq_number: i32, message: i32) {
        todo!();
    }
}

impl Actor for Worker {
    type Message = WorkerMessages;

    fn receive_local(&mut self, msg: Self::Message) -> Handled {
        match msg {
            WorkerMessages::External(master_request) => {
                /* External::WorkerStateUpdateConfirmation */
                match master_request {
                    External::MasterMessage(_) => todo!(),
                    External::WorkerRfpResponse => todo!(),
                    External::WorkerStateUpdateConfirmation => todo!(),
                }
                //     info!(self.ctx.log(), "received state update {:?}", master_request);
                // } else {
                //     println!("RFP sent through channel, not port");
                //     debug!(self.ctx.log(), "RFP sent through channel, not port")
                // }

                //match master request to rfp or accepted proposal
                //run logic relative to match
                //trigger port
                todo!();
            }
            WorkerMessages::InternalStateUpdate(state_update) => {
                self.update_state(state_update.seq_number, state_update.message);
            }
        }

        Handled::Ok
    }
    fn receive_network(&mut self, _msg: NetMessage) -> Handled {
        unimplemented!("No receive network message handling on Worker")
    }
}

impl Provide<MessagePort> for Worker {
    fn handle(&mut self, event: MasterMessage) -> Handled {
        match event {
            MasterMessage::Rfp => {
                //generate (assign to self.proposed_sequence_number) and return proposal response to Rfp Req
                todo!();
            }
            MasterMessage::AcceptedProposalBroadcast {
                seq_number,
                message,
            } => {
                self.actor_ref()
                    .tell(WorkerMessages::InternalStateUpdate(StateUpdate {
                        seq_number,
                        message,
                    }));
            }
        };
        Handled::Ok
    }
}

//
// #[derive(Debug)]
// pub enum InternalCommand {
//     UpdateState { seq_num: i32, message: i32 },
//     Start,
//     Stop,
//     Kill,
// }
// //Response status is for handling worker internal state prior to receivng/sending responses to Master
// #[derive(Debug, Clone)]
// enum ResponseStatus {
//     Proposal,
//     Acknowledged,
// }
//
// fn default_response() -> WorkerResponse {
//     WorkerResponse {
//         proposed_sequence_number: None,
//         msg: None,
//         status: ResponseStatus::Acknowledged,
//     }
// }
