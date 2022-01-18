use bytes::{BufMut, BytesMut};

use crate::command::common::PbToBytes;
use crate::pb;
use crate::protocol::packet::Packet;

impl super::super::super::Engine {
    // OidbSvc.0x88d_0
    pub fn build_group_info_request_packet(&self, group_code: i64) -> Packet {
        let transport = &self.transport;
        let body = pb::oidb::D88dReqBody {
            app_id: Some(transport.version.app_id),
            req_group_info: vec![pb::oidb::ReqGroupInfo {
                group_code: Some(group_code as u64),
                stgroupinfo: Some(pb::oidb::D88dGroupInfo {
                    group_owner: Some(0),
                    group_uin: Some(0),
                    group_create_time: Some(0),
                    group_flag: Some(0),
                    group_member_max_num: Some(0),
                    group_member_num: Some(0),
                    group_option: Some(0),
                    group_level: Some(0),
                    group_face: Some(0),
                    group_name: Some(vec![]),
                    group_memo: Some(vec![]),
                    group_finger_memo: Some(vec![]),
                    group_last_msg_time: Some(0),
                    group_cur_msg_seq: Some(0),
                    group_question: Some(vec![]),
                    group_answer: Some(vec![]),
                    group_grade: Some(0),
                    active_member_num: Some(0),
                    head_portrait_seq: Some(0),
                    msg_head_portrait: Some(pb::oidb::D88dGroupHeadPortrait::default()),
                    st_group_ex_info: Some(pb::oidb::D88dGroupExInfoOnly::default()),
                    group_sec_level: Some(0),
                    cmduin_privilege: Some(0),
                    no_finger_open_flag: Some(0),
                    no_code_finger_open_flag: Some(0),
                    ..Default::default()
                }),
                ..Default::default()
            }],
            pc_client_version: Some(0),
        };
        let payload = pb::oidb::OidbssoPkg {
            command: 2189,
            bodybuffer: body.to_bytes().to_vec(),
            ..Default::default()
        };
        self.uni_packet("OidbSvc.0x88d_0", payload.to_bytes())
    }

    // OidbSvc.0x570_8
    pub fn build_group_mute_packet(
        &self,
        group_code: i64,
        member_uin: i64,
        duration: u32,
    ) -> Packet {
        let mut w = BytesMut::new();
        w.put_u32(group_code as u32);
        w.put_u8(32);
        w.put_u16(1);
        w.put_u32(member_uin as u32);
        w.put_u32(duration);
        let payload = self.transport.encode_oidb_packet(0x570, 8, w.freeze());
        self.uni_packet("OidbSvc.0x570_8", payload)
    }

    // OidbSvc.0x89a_0
    fn build_group_operation_packet(&self, body: pb::oidb::D89aReqBody) -> Packet {
        let payload = self.transport.encode_oidb_packet(0x89a, 0, body.to_bytes());
        self.uni_packet("OidbSvc.0x89a_0", payload)
    }

    // OidbSvc.0x89a_0
    pub fn build_group_mute_all_packet(&self, group_code: i64, mute: bool) -> Packet {
        let shut_up_time: i32 = if mute { 268435455 } else { 0 };
        let body = pb::oidb::D89aReqBody {
            group_code,
            st_group_info: Some(pb::oidb::D89aGroupinfo {
                shutup_time: Some(pb::oidb::d89a_groupinfo::ShutupTime::Val(shut_up_time)),
                ..Default::default()
            }),
            ..Default::default()
        };
        self.build_group_operation_packet(body)
    }

    // OidbSvc.0x89a_0
    pub fn build_group_name_update_packet(&self, group_code: i64, name: String) -> Packet {
        let body = pb::oidb::D89aReqBody {
            group_code,
            st_group_info: Some(pb::oidb::D89aGroupinfo {
                ing_group_name: name.as_bytes().to_vec(),
                ..Default::default()
            }),
            ..Default::default()
        };
        self.build_group_operation_packet(body)
    }

    // OidbSvc.0x89a_0
    pub fn build_group_memo_update_packet(&self, group_code: i64, memo: String) -> Packet {
        let body = pb::oidb::D89aReqBody {
            group_code,
            st_group_info: Some(pb::oidb::D89aGroupinfo {
                ing_group_memo: memo.as_bytes().to_vec(),
                ..Default::default()
            }),
            ..Default::default()
        };
        self.build_group_operation_packet(body)
    }

    // OidbSvc.0x8a0_0
    pub fn build_group_kick_packet(
        &self,
        group_code: i64,
        member_uin: i64,
        kick_msg: &str,
        block: bool,
    ) -> Packet {
        let flag_block = if block { 1 } else { 0 };
        let body = pb::oidb::D8a0ReqBody {
            opt_uint64_group_code: group_code,
            msg_kick_list: vec![pb::oidb::D8a0KickMemberInfo {
                opt_uint32_operate: 5,
                opt_uint64_member_uin: member_uin,
                opt_uint32_flag: flag_block,
                ..Default::default()
            }],
            kick_msg: kick_msg.as_bytes().to_vec(),
            ..Default::default()
        };
        let payload = self.transport.encode_oidb_packet(0x8a0, 0, body.to_bytes());
        self.uni_packet("OidbSvc.0x8a0_0", payload)
    }

    // OidbSvc.0xed3
    pub fn build_group_poke_packet(&self, group_code: i64, target: i64) -> Packet {
        let body = pb::oidb::Ded3ReqBody {
            to_uin: target,
            group_code,
            ..Default::default()
        };
        let payload = self.transport.encode_oidb_packet(0xed3, 1, body.to_bytes());
        self.uni_packet("OidbSvc.0xed3", payload)
    }

    // OidbSvc.0xed3
    pub fn build_friend_poke_packet(&self, target: i64) -> Packet {
        let body = pb::oidb::Ded3ReqBody {
            to_uin: target,
            aio_uin: target,
            ..Default::default()
        };
        let payload = self.transport.encode_oidb_packet(0xed3, 1, body.to_bytes());
        self.uni_packet("OidbSvc.0xed3", payload)
    }

    // OidbSvc.0x55c_1
    pub fn build_group_admin_set_packet(&self, group_code: i64, member: i64, flag: bool) -> Packet {
        let mut w = BytesMut::new();
        w.put_u32(group_code as u32);
        w.put_u32(member as u32);
        w.put_u8(if flag { 0x01 } else { 0x00 });
        let payload = self.transport.encode_oidb_packet(0x55c, 1, w.freeze());
        self.uni_packet("OidbSvc.0x55c_1", payload)
    }

    // OidbSvc.0x758
    pub fn build_group_invite_packet(&self, group_code: i64, uin: i64) -> Packet {
        let body = pb::oidb::D758ReqBody {
            join_group_code: Some(group_code as u64),
            be_invited_uin_info: vec![pb::oidb::InviteUinInfo {
                uin: Some(uin as u64),
                ..Default::default()
            }],
            ..Default::default()
        };
        let payload = self.transport.encode_oidb_packet(0x758, 1, body.to_bytes());
        self.uni_packet("OidbSvc.0x758", payload)
    }

    // OidbSvc.0x8a7_0
    pub fn build_group_at_all_remain_request_packet(&self, group_code: i64) -> Packet {
        let body = pb::oidb::D8a7ReqBody {
            sub_cmd: Some(1),
            limit_interval_type_for_uin: Some(2),
            limit_interval_type_for_group: Some(1),
            uin: Some(self.uin() as u64),
            group_code: Some(group_code as u64),
        };
        let payload = self.transport.encode_oidb_packet(0x8a7, 0, body.to_bytes());
        self.uni_packet("OidbSvc.0x8a7_0", payload)
    }
}
