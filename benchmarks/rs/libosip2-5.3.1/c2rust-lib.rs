#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(rustc_private)]


extern crate libc;
pub mod src {
pub mod osip2 {
pub mod fsm_misc;
pub mod ict;
pub mod ict_fsm;
pub mod ist;
pub mod ist_fsm;
pub mod nict;
pub mod nict_fsm;
pub mod nist;
pub mod nist_fsm;
pub mod osip;
pub mod osip_dialog;
pub mod osip_event;
pub mod osip_time;
pub mod osip_transaction;
pub mod port_condv;
pub mod port_fifo;
pub mod port_sema;
pub mod port_thread;
} // mod osip2
pub mod osipparser2 {
pub mod osip_accept;
pub mod osip_accept_encoding;
pub mod osip_accept_language;
pub mod osip_alert_info;
pub mod osip_allow;
pub mod osip_authentication_info;
pub mod osip_authorization;
pub mod osip_body;
pub mod osip_call_id;
pub mod osip_call_info;
pub mod osip_contact;
pub mod osip_content_disposition;
pub mod osip_content_encoding;
pub mod osip_content_length;
pub mod osip_content_type;
pub mod osip_cseq;
pub mod osip_error_info;
pub mod osip_from;
pub mod osip_header;
pub mod osip_list;
pub mod osip_md5c;
pub mod osip_message;
pub mod osip_message_parse;
pub mod osip_message_to_str;
pub mod osip_mime_version;
pub mod osip_parser_cfg;
pub mod osip_port;
pub mod osip_proxy_authenticate;
pub mod osip_proxy_authentication_info;
pub mod osip_proxy_authorization;
pub mod osip_record_route;
pub mod osip_route;
pub mod osip_to;
pub mod osip_uri;
pub mod osip_via;
pub mod osip_www_authenticate;
pub mod sdp_accessor;
pub mod sdp_message;
} // mod osipparser2
pub mod test {
pub mod tcallid;
pub mod tcontact;
pub mod tcontentt;
pub mod tfrom;
pub mod torture;
pub mod trecordr;
pub mod troute;
pub mod tto;
pub mod turls;
pub mod tvia;
pub mod twwwa;
} // mod test
} // mod src
