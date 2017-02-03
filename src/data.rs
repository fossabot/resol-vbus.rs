use std::cmp::Ordering::{self, Less, Equal, Greater};

use header::Header;
use packet::Packet;
use datagram::Datagram;
use telegram::Telegram;


/// `Data` is a type that contains one of the supported VBus protocol data variants.
#[derive(Clone, Debug)]
pub enum Data {
    /// Contains a `Packet` conforming to VBus protocol version 1.x.
    Packet(Packet),

    /// Contains a `Datagram` conforming to VBus protocol version 2.x.
    Datagram(Datagram),

    /// Contains a `Telegram` conforming to VBus protocol version 3.x.
    Telegram(Telegram),
}


impl Data {

    /// Returns the `Header` part of the variant inside this `Data`.
    pub fn as_header(&self) -> &Header {
        match *self {
            Data::Packet(ref packet) => &packet.header,
            Data::Datagram(ref dgram) => &dgram.header,
            Data::Telegram(ref tgram) => &tgram.header,
        }
    }

    /// Creates an ID string for the variant inside this `Data`.
    pub fn to_id_string(&self) -> String {
        match *self {
            Data::Packet(ref packet) => packet.to_id_string(),
            Data::Datagram(ref dgram) => dgram.to_id_string(),
            Data::Telegram(ref tgram) => tgram.to_id_string(),
        }
    }

}


impl PartialEq for Data {

    /// Compare two `Data` objects for equality in context of a `DataSet`.
    fn eq(&self, right: &Data) -> bool {
        let left = self;

        let left_header = left.as_header();
        let right_header = right.as_header();

        if left_header.channel != right_header.channel {
            false
        } else if left_header.destination_address != right_header.destination_address {
            false
        } else if left_header.source_address != right_header.source_address {
            false
        } else if left_header.protocol_version != right_header.protocol_version {
            false
        } else {
            match *left {
                Data::Packet(ref left_packet) => {
                    if let Data::Packet(ref right_packet) = *right {
                        if left_packet.command != right_packet.command {
                            false
                        } else {
                            true
                        }
                    } else {
                        false
                    }
                }
                Data::Datagram(ref left_dgram) => {
                    if let Data::Datagram(ref right_dgram) = *right {
                        if left_dgram.command != right_dgram.command {
                            false
                        } else if left_dgram.command != 0x0900 {
                            true
                        } else if left_dgram.param16 != right_dgram.param16 {
                            false
                        } else {
                            true
                        }
                    } else {
                        false
                    }
                }
                Data::Telegram(ref left_tgram) => {
                    if let Data::Telegram(ref right_tgram) = *right {
                        if left_tgram.command != right_tgram.command {
                            false
                        } else {
                            true
                        }
                    } else {
                        false
                    }
                }
            }
        }
    }

}


impl PartialOrd for Data {

    fn partial_cmp(&self, right: &Data) -> Option<Ordering> {
        let left = self;

        let left_header = left.as_header();
        let right_header = right.as_header();

        if left_header.channel < right_header.channel {
            Some(Less)
        } else if left_header.channel > right_header.channel {
            Some(Greater)
        } else if left_header.destination_address < right_header.destination_address {
            Some(Less)
        } else if left_header.destination_address > right_header.destination_address {
            Some(Greater)
        } else if left_header.source_address < right_header.source_address {
            Some(Less)
        } else if left_header.source_address > right_header.source_address {
            Some(Greater)
        } else if left_header.protocol_version < right_header.protocol_version {
            Some(Less)
        } else if left_header.protocol_version > right_header.protocol_version {
            Some(Greater)
        } else {
            match *left {
                Data::Packet(ref left_packet) => {
                    if let Data::Packet(ref right_packet) = *right {
                        if left_packet.command < right_packet.command {
                            Some(Less)
                        } else if left_packet.command > right_packet.command {
                            Some(Greater)
                        } else {
                            Some(Equal)
                        }
                    } else {
                        None
                    }
                }
                Data::Datagram(ref left_dgram) => {
                    if let Data::Datagram(ref right_dgram) = *right {
                        if left_dgram.command < right_dgram.command {
                            Some(Less)
                        } else if left_dgram.command > right_dgram.command {
                            Some(Greater)
                        } else if left_dgram.command != 0x0900 {
                            Some(Equal)
                        } else if left_dgram.param16 < right_dgram.param16 {
                            Some(Less)
                        } else if left_dgram.param16 > right_dgram.param16 {
                            Some(Greater)
                        } else {
                            Some(Equal)
                        }
                    } else {
                        None
                    }
                }
                Data::Telegram(ref left_tgram) => {
                    if let Data::Telegram(ref right_tgram) = *right {
                        if left_tgram.command < right_tgram.command {
                            Some(Less)
                        } else if left_tgram.command > right_tgram.command {
                            Some(Greater)
                        } else {
                            Some(Equal)
                        }
                    } else {
                        None
                    }
                }
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, UTC};

    use live_data_decoder::data_from_checked_bytes;

    use super::*;

    use test_data::{LIVE_DATA_1, LIVE_TELEGRAM_1};

    #[test]
    fn test_as_header() {
        let timestamp = UTC.timestamp(1485688933, 0);
        let channel = 0x11;

        let packet_data = data_from_checked_bytes(timestamp, channel, &LIVE_DATA_1 [0..]);

        let header = packet_data.as_header();
        assert_eq!(timestamp, header.timestamp);
        assert_eq!(channel, header.channel);
        assert_eq!(0x0010, header.destination_address);
        assert_eq!(0x7E11, header.source_address);
        assert_eq!(0x10, header.protocol_version);

        let dgram_data = data_from_checked_bytes(timestamp, channel, &LIVE_DATA_1 [352..]);

        let header = dgram_data.as_header();
        assert_eq!(timestamp, header.timestamp);
        assert_eq!(channel, header.channel);
        assert_eq!(0x0000, header.destination_address);
        assert_eq!(0x7E11, header.source_address);
        assert_eq!(0x20, header.protocol_version);

        let tgram_data = data_from_checked_bytes(timestamp, channel, &LIVE_TELEGRAM_1 [0..]);

        let header = tgram_data.as_header();
        assert_eq!(timestamp, header.timestamp);
        assert_eq!(channel, header.channel);
        assert_eq!(0x7771, header.destination_address);
        assert_eq!(0x2011, header.source_address);
        assert_eq!(0x30, header.protocol_version);
    }

    #[test]
    fn test_eq() {
        let timestamp = UTC.timestamp(1485688933, 0);
        let channel = 0x11;

        let packet_data = data_from_checked_bytes(timestamp, channel, &LIVE_DATA_1 [0..]);
        let packet = match packet_data {
            Data::Packet(ref packet) => packet,
            _ => unreachable!(),
        };

        let dgram_data = data_from_checked_bytes(timestamp, channel, &LIVE_DATA_1 [352..]);
        let dgram = match dgram_data {
            Data::Datagram(ref dgram) => dgram,
            _ => unreachable!(),
        };

        let tgram_data = data_from_checked_bytes(timestamp, channel, &LIVE_TELEGRAM_1 [0..]);
        let tgram = match tgram_data {
            Data::Telegram(ref tgram) => tgram,
            _ => unreachable!(),
        };

        let other_timestamp = UTC.timestamp(0, 0);

        // Between variants
        assert_eq!(false, packet_data.eq(&dgram_data));
        assert_eq!(false, packet_data.eq(&tgram_data));
        assert_eq!(false, dgram_data.eq(&tgram_data));

        // ---- Packet ----
        let other = packet.clone();
        assert_eq!(true, Data::Packet(other).eq(&packet_data));

        let mut other = packet.clone();
        other.header.timestamp = other_timestamp;
        assert_eq!(true, Data::Packet(other).eq(&packet_data));

        let mut other = packet.clone();
        other.header.channel ^= 1;
        assert_eq!(false, Data::Packet(other).eq(&packet_data));

        let mut other = packet.clone();
        other.header.destination_address ^= 1;
        assert_eq!(false, Data::Packet(other).eq(&packet_data));

        let mut other = packet.clone();
        other.header.source_address ^= 1;
        assert_eq!(false, Data::Packet(other).eq(&packet_data));

        let mut other = packet.clone();
        other.header.protocol_version ^= 1;
        assert_eq!(false, Data::Packet(other).eq(&packet_data));

        let mut other = packet.clone();
        other.command ^= 1;
        assert_eq!(false, Data::Packet(other).eq(&packet_data));

        let mut other = packet.clone();
        other.frame_count ^= 1;
        assert_eq!(true, Data::Packet(other).eq(&packet_data));

        let mut other = packet.clone();
        other.frame_data [0] ^= 1;
        assert_eq!(true, Data::Packet(other).eq(&packet_data));

        // ---- Datagram ----
        let other = dgram.clone();
        assert_eq!(true, Data::Datagram(other).eq(&dgram_data));

        let mut other = dgram.clone();
        other.header.timestamp = other_timestamp;
        assert_eq!(true, Data::Datagram(other).eq(&dgram_data));

        let mut other = dgram.clone();
        other.header.channel ^= 1;
        assert_eq!(false, Data::Datagram(other).eq(&dgram_data));

        let mut other = dgram.clone();
        other.header.destination_address ^= 1;
        assert_eq!(false, Data::Datagram(other).eq(&dgram_data));

        let mut other = dgram.clone();
        other.header.source_address ^= 1;
        assert_eq!(false, Data::Datagram(other).eq(&dgram_data));

        let mut other = dgram.clone();
        other.header.protocol_version ^= 1;
        assert_eq!(false, Data::Datagram(other).eq(&dgram_data));

        let mut other = dgram.clone();
        other.command ^= 1;
        assert_eq!(false, Data::Datagram(other).eq(&dgram_data));

        let mut other = dgram.clone();
        other.param16 ^= 1;
        assert_eq!(true, Data::Datagram(other).eq(&dgram_data));

        let mut other = dgram.clone();
        other.param32 ^= 1;
        assert_eq!(true, Data::Datagram(other).eq(&dgram_data));

        // ---- Telegram ----
        let other = tgram.clone();
        assert_eq!(true, Data::Telegram(other).eq(&tgram_data));

        let mut other = tgram.clone();
        other.header.timestamp = other_timestamp;
        assert_eq!(true, Data::Telegram(other).eq(&tgram_data));

        let mut other = tgram.clone();
        other.header.channel ^= 1;
        assert_eq!(false, Data::Telegram(other).eq(&tgram_data));

        let mut other = tgram.clone();
        other.header.destination_address ^= 1;
        assert_eq!(false, Data::Telegram(other).eq(&tgram_data));

        let mut other = tgram.clone();
        other.header.source_address ^= 1;
        assert_eq!(false, Data::Telegram(other).eq(&tgram_data));

        let mut other = tgram.clone();
        other.header.protocol_version ^= 1;
        assert_eq!(false, Data::Telegram(other).eq(&tgram_data));

        let mut other = tgram.clone();
        other.command ^= 1;
        assert_eq!(false, Data::Telegram(other).eq(&tgram_data));

        let mut other = tgram.clone();
        other.frame_data [0] ^= 1;
        assert_eq!(true, Data::Telegram(other).eq(&tgram_data));
    }

    #[test]
    fn test_partial_cmp() {
        let timestamp = UTC.timestamp(1485688933, 0);
        let channel = 0x11;

        let packet_data = data_from_checked_bytes(timestamp, channel, &LIVE_DATA_1 [0..]);
        let packet = match packet_data {
            Data::Packet(ref packet) => packet,
            _ => unreachable!(),
        };

        let dgram_data = data_from_checked_bytes(timestamp, channel, &LIVE_DATA_1 [352..]);
        let dgram = match dgram_data {
            Data::Datagram(ref dgram) => dgram,
            _ => unreachable!(),
        };

        let tgram_data = data_from_checked_bytes(timestamp, channel, &LIVE_TELEGRAM_1 [0..]);
        let tgram = match tgram_data {
            Data::Telegram(ref tgram) => tgram,
            _ => unreachable!(),
        };

        let other_timestamp = UTC.timestamp(0, 0);

        // Between variants
        assert_eq!(Some(Greater), packet_data.partial_cmp(&dgram_data));
        assert_eq!(Some(Less), packet_data.partial_cmp(&tgram_data));
        assert_eq!(Some(Less), dgram_data.partial_cmp(&tgram_data));

        // ---- Packet ----
        let other = packet.clone();
        assert_eq!(Some(Equal), Data::Packet(other).partial_cmp(&packet_data));

        let mut other = packet.clone();
        other.header.timestamp = other_timestamp;
        assert_eq!(Some(Equal), Data::Packet(other).partial_cmp(&packet_data));

        let mut other = packet.clone();
        other.header.channel -= 1;
        assert_eq!(Some(Less), Data::Packet(other).partial_cmp(&packet_data));

        let mut other = packet.clone();
        other.header.channel += 1;
        assert_eq!(Some(Greater), Data::Packet(other).partial_cmp(&packet_data));

        let mut other = packet.clone();
        other.header.destination_address -= 1;
        assert_eq!(Some(Less), Data::Packet(other).partial_cmp(&packet_data));

        let mut other = packet.clone();
        other.header.destination_address += 1;
        assert_eq!(Some(Greater), Data::Packet(other).partial_cmp(&packet_data));

        let mut other = packet.clone();
        other.header.source_address -= 1;
        assert_eq!(Some(Less), Data::Packet(other).partial_cmp(&packet_data));

        let mut other = packet.clone();
        other.header.source_address += 1;
        assert_eq!(Some(Greater), Data::Packet(other).partial_cmp(&packet_data));

        let mut other = packet.clone();
        other.header.protocol_version -= 1;
        assert_eq!(Some(Less), Data::Packet(other).partial_cmp(&packet_data));

        let mut other = packet.clone();
        other.header.protocol_version += 1;
        assert_eq!(Some(Greater), Data::Packet(other).partial_cmp(&packet_data));

        let mut other = packet.clone();
        other.command -= 1;
        assert_eq!(Some(Less), Data::Packet(other).partial_cmp(&packet_data));

        let mut other = packet.clone();
        other.command += 1;
        assert_eq!(Some(Greater), Data::Packet(other).partial_cmp(&packet_data));

        let mut other = packet.clone();
        other.frame_count ^= 1;
        assert_eq!(Some(Equal), Data::Packet(other).partial_cmp(&packet_data));

        let mut other = packet.clone();
        other.frame_data [0] ^= 1;
        assert_eq!(Some(Equal), Data::Packet(other).partial_cmp(&packet_data));

        // ---- Datagram ----
        let other = dgram.clone();
        assert_eq!(Some(Equal), Data::Datagram(other).partial_cmp(&dgram_data));

        let mut other = dgram.clone();
        other.header.timestamp = other_timestamp;
        assert_eq!(Some(Equal), Data::Datagram(other).partial_cmp(&dgram_data));

        let mut other = dgram.clone();
        other.header.channel -= 1;
        assert_eq!(Some(Less), Data::Datagram(other).partial_cmp(&dgram_data));

        let mut other = dgram.clone();
        other.header.channel += 1;
        assert_eq!(Some(Greater), Data::Datagram(other).partial_cmp(&dgram_data));

        let mut other = dgram.clone();
        other.header.destination_address += 1; // NOTE(daniel): subtraction would underflow
        assert_eq!(Some(Less), dgram_data.partial_cmp(&Data::Datagram(other)));

        let mut other = dgram.clone();
        other.header.destination_address += 1;
        assert_eq!(Some(Greater), Data::Datagram(other).partial_cmp(&dgram_data));

        let mut other = dgram.clone();
        other.header.source_address -= 1;
        assert_eq!(Some(Less), Data::Datagram(other).partial_cmp(&dgram_data));

        let mut other = dgram.clone();
        other.header.source_address += 1;
        assert_eq!(Some(Greater), Data::Datagram(other).partial_cmp(&dgram_data));

        let mut other = dgram.clone();
        other.header.protocol_version -= 1;
        assert_eq!(Some(Less), Data::Datagram(other).partial_cmp(&dgram_data));

        let mut other = dgram.clone();
        other.header.protocol_version += 1;
        assert_eq!(Some(Greater), Data::Datagram(other).partial_cmp(&dgram_data));

        let mut other = dgram.clone();
        other.command -= 1;
        assert_eq!(Some(Less), Data::Datagram(other).partial_cmp(&dgram_data));

        let mut other = dgram.clone();
        other.command += 1;
        assert_eq!(Some(Greater), Data::Datagram(other).partial_cmp(&dgram_data));

        let mut other = dgram.clone();
        other.param16 ^= 1;
        assert_eq!(Some(Equal), Data::Datagram(other).partial_cmp(&dgram_data));

        let mut other = dgram.clone();
        other.param32 ^= 1;
        assert_eq!(Some(Equal), Data::Datagram(other).partial_cmp(&dgram_data));

        // ---- Telegram ----
        let other = tgram.clone();
        assert_eq!(Some(Equal), Data::Telegram(other).partial_cmp(&tgram_data));

        let mut other = tgram.clone();
        other.header.timestamp = other_timestamp;
        assert_eq!(Some(Equal), Data::Telegram(other).partial_cmp(&tgram_data));

        let mut other = tgram.clone();
        other.header.channel -= 1;
        assert_eq!(Some(Less), Data::Telegram(other).partial_cmp(&tgram_data));

        let mut other = tgram.clone();
        other.header.channel += 1;
        assert_eq!(Some(Greater), Data::Telegram(other).partial_cmp(&tgram_data));

        let mut other = tgram.clone();
        other.header.destination_address -= 1;
        assert_eq!(Some(Less), Data::Telegram(other).partial_cmp(&tgram_data));

        let mut other = tgram.clone();
        other.header.destination_address += 1;
        assert_eq!(Some(Greater), Data::Telegram(other).partial_cmp(&tgram_data));

        let mut other = tgram.clone();
        other.header.source_address -= 1;
        assert_eq!(Some(Less), Data::Telegram(other).partial_cmp(&tgram_data));

        let mut other = tgram.clone();
        other.header.source_address += 1;
        assert_eq!(Some(Greater), Data::Telegram(other).partial_cmp(&tgram_data));

        let mut other = tgram.clone();
        other.header.protocol_version -= 1;
        assert_eq!(Some(Less), Data::Telegram(other).partial_cmp(&tgram_data));

        let mut other = tgram.clone();
        other.header.protocol_version += 1;
        assert_eq!(Some(Greater), Data::Telegram(other).partial_cmp(&tgram_data));

        let mut other = tgram.clone();
        other.command -= 1;
        assert_eq!(Some(Less), Data::Telegram(other).partial_cmp(&tgram_data));

        let mut other = tgram.clone();
        other.command += 1;
        assert_eq!(Some(Greater), Data::Telegram(other).partial_cmp(&tgram_data));

        let mut other = tgram.clone();
        other.frame_data [0] ^= 1;
        assert_eq!(Some(Equal), Data::Telegram(other).partial_cmp(&tgram_data));
    }
}
