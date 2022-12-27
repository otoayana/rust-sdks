use crate::proto::{data_packet, DataPacket, UserPacket};
use crate::room::participant::{
    impl_participant_trait, ParticipantInternalTrait, ParticipantShared, ParticipantTrait,
};
use crate::room::{RoomError, RoomEmitter};
use crate::rtc_engine::RTCEngine;

#[derive(Debug)]
pub struct LocalParticipant {
    shared: ParticipantShared,
    rtc_engine: Arc<RTCEngine>,
}

impl LocalParticipant {
    pub(crate) fn new(
        rtc_engine: Arc<RTCEngine>,
        sid: ParticipantSid,
        identity: ParticipantIdentity,
        name: String,
        metadata: String,
        room_emitter: RoomEmitter,
    ) -> Self {
        Self {
            shared: ParticipantShared::new(sid, identity, name, metadata, room_emitter),
            rtc_engine,
        }
    }

    pub async fn publish_data(
        &self,
        data: &[u8],
        kind: data_packet::Kind,
    ) -> Result<(), RoomError> {
        let data = DataPacket {
            kind: kind as i32,
            value: Some(data_packet::Value::User(UserPacket {
                participant_sid: self.sid().to_string(),
                payload: data.to_vec(),
                destination_sids: vec![],
            })),
        };

        self.rtc_engine
            .publish_data(&data, kind)
            .await
            .map_err(Into::into)
    }
}

impl ParticipantInternalTrait for LocalParticipant {
    fn update_info(&self, info: ParticipantInfo) {
        self.shared.update_info(info);
    }
}

impl_participant_trait!(LocalParticipant);
