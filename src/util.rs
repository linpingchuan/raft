use slog::{OwnedKVList, Record, KV};
use std::fmt;
use std::fmt::Write;
use std::u64;

use crate::protos::eraftpb::{Entry, Message};

/// A number to represent that there is no limit.
pub const NO_LIMIT: u64 = u64::MAX;

struct FormatKeyValueList {
    pub buffer: String,
}

impl slog::Serializer for FormatKeyValueList {
    fn emit_arguments(&mut self, key: slog::Key, val: &fmt::Arguments) -> slog::Result {
        if !self.buffer.is_empty() {
            write!(&mut self.buffer, ", {}: {}", key, val).unwrap();
        } else {
            write!(&mut self.buffer, "{}: {}", key, val).unwrap();
        }
        Ok(())
    }
}

pub(crate) fn format_kv_list(kv_list: &OwnedKVList) -> String {
    let mut formatter = FormatKeyValueList {
        buffer: "".to_owned(),
    };
    let record = record_static!(slog::Level::Trace, "");
    kv_list
        .serialize(
            &Record::new(&record, &format_args!(""), b!()),
            &mut formatter,
        )
        .unwrap();
    formatter.buffer
}
