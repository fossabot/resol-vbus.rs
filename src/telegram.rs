use std::fmt::{Debug, Error, Formatter};

use header::Header;


/// The `Telegram` type stores information according to the VBus protocol version 3.x.
pub struct Telegram {
    /// The shared `Header` of all VBus protocol types.
    pub header: Header,

    /// The command of this `Telegram`.
    pub command: u8,

    /// The actual data from the frames attached to this `Telegram`.
    pub frame_data: [u8; 21],
}


impl Telegram {

    /// Get number of frames from a VBus protocol version 3.x command.
    pub fn frame_count_for_command(command: u8) -> u8 {
        command >> 5
    }

    /// Get number of 7-byte frames attached to this `Telegram`.
    pub fn frame_count(&self) -> u8 {
        Telegram::frame_count_for_command(self.command)
    }

    /// Creates an ID string for this `Telegram`.
    pub fn to_id_string(&self) -> String {
        format!("{}_{:02X}", self.header.to_id_string(), self.command)
    }

}


impl Debug for Telegram {

    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_fmt(format_args!("Telegram {{ header: {:?}, command: 0x{:02X}, frame_data: ... }}", self.header, self.command))
    }

}


#[cfg(test)]
mod tests {
    use chrono::{TimeZone, UTC};

    use header::Header;

    use super::*;

    #[test]
    fn test_frame_count_for_command() {
        assert_eq!(0, Telegram::frame_count_for_command(0x1F));
        assert_eq!(1, Telegram::frame_count_for_command(0x3F));
        assert_eq!(2, Telegram::frame_count_for_command(0x5F));
        assert_eq!(3, Telegram::frame_count_for_command(0x7F));
    }

    #[test]
    fn test_frame_count() {
        let timestamp = UTC.timestamp(1485688933, 0);

        let frame_data = [0u8; 21];

        let tgram = Telegram {
            header: Header {
                timestamp: timestamp,
                channel: 0x11,
                destination_address: 0x1213,
                source_address: 0x1415,
                protocol_version: 0x36,
            },
            command: 0x37,
            frame_data: frame_data,
        };

        assert_eq!(1, tgram.frame_count());
    }

    #[test]
    fn test_to_id_string() {
        let timestamp = UTC.timestamp(1485688933, 0);

        let frame_data = [0u8; 21];

        let tgram = Telegram {
            header: Header {
                timestamp: timestamp,
                channel: 0x11,
                destination_address: 0x1213,
                source_address: 0x1415,
                protocol_version: 0x36,
            },
            command: 0x17,
            frame_data: frame_data,
        };

        assert_eq!("11_1213_1415_36_17", tgram.to_id_string());
    }

    #[test]
    fn test_debug_fmt() {
        let timestamp = UTC.timestamp(1485688933, 0);

        let frame_data = [0u8; 21];

        let tgram = Telegram {
            header: Header {
                timestamp: timestamp,
                channel: 0x11,
                destination_address: 0x1213,
                source_address: 0x1415,
                protocol_version: 0x26,
            },
            command: 0x17,
            frame_data: frame_data,
        };

        let result = format!("{:?}", tgram);

        assert_eq!("Telegram { header: Header { timestamp: 2017-01-29T11:22:13Z, channel: 0x11, destination_address: 0x1213, source_address: 0x1415, protocol_version: 0x26 }, command: 0x17, frame_data: ... }", result);
    }
}