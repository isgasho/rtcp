#![warn(rust_2018_idioms)]
#![allow(dead_code)]

//! Package rtcp implements encoding and decoding of RTCP packets according to RFCs 3550 and 5506.
//!
//! RTCP is a sister protocol of the Real-time Transport Protocol (RTP). Its basic functionality
//! and packet structure is defined in RFC 3550. RTCP provides out-of-band statistics and control
//! information for an RTP session. It partners with RTP in the delivery and packaging of multimedia data,
//! but does not transport any media data itself.
//!
//! The primary function of RTCP is to provide feedback on the quality of service (QoS)
//! in media distribution by periodically sending statistics information such as transmitted octet
//! and packet counts, packet loss, packet delay variation, and round-trip delay time to participants
//! in a streaming multimedia session. An application may use this information to control quality of
//! service parameters, perhaps by limiting flow, or using a different codec.
//!
//! Decoding RTCP packets:
//!```nobuild
//!     let pkt = rtcp.Unmarshal(rtcpData).unwrap();
//!
//!     if let Some(e) = packet
//!          .as_any()
//!          .downcast_ref::<crate::picture_loss_indication::PictureLossIndication>()
//!      {
//!    
//!      }
//!     else if let Some(e) = packet
//!          .as_any()
//!          .downcast_ref::<crate::goodbye::Goodbye>(){}
//!     ....
//!
//!
//! Encoding RTCP packets:
//!```nobuild
//!     let pkt = crate::picture_loss_indication::PictureLossIndication{
//!         sender_ssrc: sender_ssrc,
//!         media_ssrc: media_ssrc
//!     };
//!
//!     let pliData = pkt.marshal().unwrap();
//!     // ...

#[macro_use]
extern crate lazy_static;

pub mod compound_packet;
pub mod errors;
pub mod full_intra_request;
pub mod goodbye;
pub mod header;
pub mod packet;
pub mod picture_loss_indication;
pub mod rapid_resynchronization_request;
pub mod raw_packet;
pub mod receiver_estimated_maximum_bitrate;
pub mod receiver_report;
pub mod reception_report;
pub mod sender_report;
pub mod slice_loss_indication;
pub mod source_description;
pub mod transport_layer_cc;
pub mod transport_layer_nack;
mod util;
