use chrono::{DateTime, UTC};

use data::Data;


/// A `DataSet` contains a set of unique `Data`.
#[derive(Clone, Debug)]
pub struct DataSet {
    /// The timestamp that corresponds to the contained set of `Data` objects.
    pub timestamp: DateTime<UTC>,
    set: Vec<Data>,
}


impl DataSet {

    /// Construct an empty `DataSet`.
    pub fn new() -> DataSet {
        DataSet {
            timestamp: UTC::now(),
            set: Vec::new(),
        }
    }

    /// Construct a `DataSet` from a list of `Data` objects.
    pub fn from_data(timestamp: DateTime<UTC>, set: Vec<Data>) -> DataSet {
        DataSet {
            timestamp: timestamp,
            set: set,
        }
    }

    /// Return the `Data` objects contained in this `DataSet`.
    pub fn as_data_slice(&self) -> &[Data] {
        &self.set [..]
    }

    /// Add a `Data` object, replacing any equivalent existing one.
    pub fn add_data(&mut self, data: Data) {
        let timestamp = data.as_header().timestamp;

        let position = self.set.iter().position(|d| {
            d.eq(&data)
        });

        match position {
            Some(index) => self.set [index] = data,
            None => self.set.push(data),
        };

        if self.timestamp < timestamp {
            self.timestamp = timestamp;
        }
    }

    /// Add all `Data` objects from one `DataSet` into another.
    pub fn add_data_set(&mut self, data_set: DataSet) {
        let timestamp = data_set.timestamp;

        for data in data_set.set.into_iter() {
            self.add_data(data);
        }

        if self.timestamp < timestamp {
            self.timestamp = timestamp;
        }
    }

    /// Remove `Data` with timestamps older than `min_timestamp`.
    pub fn remove_data_older_than(&mut self, min_timestamp: DateTime<UTC>) {
        self.set.retain(|data| data.as_header().timestamp >= min_timestamp);
    }

    /// Sort the `Data` objects contained in this `DataSet`.
    pub fn sort(&mut self) {
        self.set.sort_by(|l, r| { l.partial_cmp(r).unwrap() });
    }

}


#[cfg(test)]
mod tests {
    use chrono::{Duration, TimeZone, UTC};

    use live_data_decoder::data_from_checked_bytes;

    use super::*;

    use test_data::{LIVE_DATA_1, LIVE_TELEGRAM_1};

    #[test]
    fn test_add_data() {
        let timestamp = UTC.timestamp(1485688933, 0);
        let channel = 0x11;

        let packet_data = data_from_checked_bytes(timestamp, channel, &LIVE_DATA_1 [0..]);
        let dgram_data = data_from_checked_bytes(timestamp, channel, &LIVE_DATA_1 [352..]);
        let tgram_data = data_from_checked_bytes(timestamp, channel, &LIVE_TELEGRAM_1 [0..]);

        let mut data_set = DataSet::new();
        data_set.timestamp = UTC.timestamp(0, 0);
        assert_eq!(0, data_set.as_data_slice().len());

        data_set.add_data(packet_data.clone());
        assert_eq!(timestamp, data_set.timestamp);
        assert_eq!(1, data_set.as_data_slice().len());
        assert_eq!("11_0010_7E11_10_0100", data_set.as_data_slice() [0].to_id_string());

        let other_timestamp = timestamp + Duration::seconds(1);

        let data = data_from_checked_bytes(other_timestamp, channel, &LIVE_DATA_1 [0..]);
        data_set.add_data(data);
        assert_eq!(other_timestamp, data_set.timestamp);
        assert_eq!(1, data_set.as_data_slice().len());
        assert_eq!("11_0010_7E11_10_0100", data_set.as_data_slice() [0].to_id_string());

        let other_channel = channel + 1;

        let data = data_from_checked_bytes(timestamp, other_channel, &LIVE_DATA_1 [0..]);
        data_set.add_data(data);
        assert_eq!(other_timestamp, data_set.timestamp);
        assert_eq!(2, data_set.as_data_slice().len());
        assert_eq!("11_0010_7E11_10_0100", data_set.as_data_slice() [0].to_id_string());
        assert_eq!("12_0010_7E11_10_0100", data_set.as_data_slice() [1].to_id_string());

        data_set.add_data(dgram_data.clone());
        assert_eq!(other_timestamp, data_set.timestamp);
        assert_eq!(3, data_set.as_data_slice().len());
        assert_eq!("11_0010_7E11_10_0100", data_set.as_data_slice() [0].to_id_string());
        assert_eq!("12_0010_7E11_10_0100", data_set.as_data_slice() [1].to_id_string());
        assert_eq!("11_0000_7E11_20_0500_0000", data_set.as_data_slice() [2].to_id_string());

        data_set.add_data(tgram_data.clone());
        assert_eq!(other_timestamp, data_set.timestamp);
        assert_eq!(4, data_set.as_data_slice().len());
        assert_eq!("11_0010_7E11_10_0100", data_set.as_data_slice() [0].to_id_string());
        assert_eq!("12_0010_7E11_10_0100", data_set.as_data_slice() [1].to_id_string());
        assert_eq!("11_0000_7E11_20_0500_0000", data_set.as_data_slice() [2].to_id_string());
        assert_eq!("11_7771_2011_30_25", data_set.as_data_slice() [3].to_id_string());
    }

    #[test]
    fn test_add_data_set() {
        let timestamp = UTC.timestamp(1485688933, 0);
        let channel = 0x11;

        let mut data_set = DataSet::new();
        data_set.timestamp = UTC.timestamp(0, 0);
        data_set.add_data(data_from_checked_bytes(timestamp, channel, &LIVE_DATA_1 [0..]));
        data_set.add_data(data_from_checked_bytes(timestamp, channel, &LIVE_DATA_1 [352..]));
        data_set.add_data(data_from_checked_bytes(timestamp, channel, &LIVE_TELEGRAM_1 [0..]));

        let mut other_data_set = DataSet::new();
        other_data_set.timestamp = UTC.timestamp(0, 0);
        other_data_set.add_data_set(data_set);

        assert_eq!(timestamp, other_data_set.timestamp);
        assert_eq!(3, other_data_set.as_data_slice().len());
        assert_eq!("11_0010_7E11_10_0100", other_data_set.as_data_slice() [0].to_id_string());
        assert_eq!("11_0000_7E11_20_0500_0000", other_data_set.as_data_slice() [1].to_id_string());
        assert_eq!("11_7771_2011_30_25", other_data_set.as_data_slice() [2].to_id_string());
    }

    #[test]
    fn test_remove_data_older_than() {
        let timestamp = UTC.timestamp(1485688933, 0);
        let channel = 0x11;

        let mut data_set = DataSet::new();
        data_set.timestamp = UTC.timestamp(0, 0);
        data_set.add_data(data_from_checked_bytes(timestamp + Duration::seconds(10), channel, &LIVE_DATA_1 [0..]));
        data_set.add_data(data_from_checked_bytes(timestamp + Duration::seconds(20), channel, &LIVE_DATA_1 [352..]));
        data_set.add_data(data_from_checked_bytes(timestamp + Duration::seconds(30), channel, &LIVE_TELEGRAM_1 [0..]));
        data_set.remove_data_older_than(timestamp + Duration::seconds(20));

        assert_eq!(timestamp + Duration::seconds(30), data_set.timestamp);
        assert_eq!(2, data_set.as_data_slice().len());
        assert_eq!("11_0000_7E11_20_0500_0000", data_set.as_data_slice() [0].to_id_string());
        assert_eq!("11_7771_2011_30_25", data_set.as_data_slice() [1].to_id_string());
    }

    #[test]
    fn test_sort() {
        let timestamp = UTC.timestamp(1485688933, 0);
        let channel = 0x11;

        let mut data_set = DataSet::new();
        data_set.timestamp = UTC.timestamp(0, 0);
        data_set.add_data(data_from_checked_bytes(timestamp, channel + 1, &LIVE_DATA_1 [0..]));
        data_set.add_data(data_from_checked_bytes(timestamp, channel, &LIVE_DATA_1 [352..]));
        data_set.add_data(data_from_checked_bytes(timestamp, channel, &LIVE_DATA_1 [0..]));
        data_set.add_data(data_from_checked_bytes(timestamp, channel, &LIVE_TELEGRAM_1 [0..]));
        data_set.add_data(data_from_checked_bytes(timestamp, channel, &LIVE_DATA_1 [258..]));
        data_set.add_data(data_from_checked_bytes(timestamp, channel, &LIVE_DATA_1 [242..]));
        data_set.add_data(data_from_checked_bytes(timestamp, channel, &LIVE_DATA_1 [172..]));

        assert_eq!(7, data_set.as_data_slice().len());
        assert_eq!("12_0010_7E11_10_0100", data_set.as_data_slice() [0].to_id_string());
        assert_eq!("11_0000_7E11_20_0500_0000", data_set.as_data_slice() [1].to_id_string());
        assert_eq!("11_0010_7E11_10_0100", data_set.as_data_slice() [2].to_id_string());
        assert_eq!("11_7771_2011_30_25", data_set.as_data_slice() [3].to_id_string());
        assert_eq!("11_6651_7E11_10_0200", data_set.as_data_slice() [4].to_id_string());
        assert_eq!("11_0010_7E22_10_0100", data_set.as_data_slice() [5].to_id_string());
        assert_eq!("11_0015_7E11_10_0100", data_set.as_data_slice() [6].to_id_string());

        data_set.sort();

        assert_eq!(7, data_set.as_data_slice().len());
        assert_eq!("11_0000_7E11_20_0500_0000", data_set.as_data_slice() [0].to_id_string());
        assert_eq!("11_0010_7E11_10_0100", data_set.as_data_slice() [1].to_id_string());
        assert_eq!("11_0010_7E22_10_0100", data_set.as_data_slice() [2].to_id_string());
        assert_eq!("11_0015_7E11_10_0100", data_set.as_data_slice() [3].to_id_string());
        assert_eq!("11_6651_7E11_10_0200", data_set.as_data_slice() [4].to_id_string());
        assert_eq!("11_7771_2011_30_25", data_set.as_data_slice() [5].to_id_string());
        assert_eq!("12_0010_7E11_10_0100", data_set.as_data_slice() [6].to_id_string());
    }
}