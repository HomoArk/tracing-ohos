//! OpenHarmony's logging backend for tracing.
use hilog_sys::{LogLevel, LogType, OH_LOG_Print, OH_LOG_IsLoggable};
use std::{
    ffi::{CStr, CString},
    io::{self, BufWriter},
    ops::Deref,
};
use tracing::Level;

fn hilog_log(level: LogLevel, domain: u16, tag: &CStr, msg: &CStr) {
    unsafe {
        OH_LOG_Print(
            LogType::LOG_APP,
            level,
            domain.into(),
            tag.as_ptr(),
            c"%{public}s".as_ptr(),
            msg.as_ptr(),
        )
    };
}


const LOGGING_TAG_MAX_LEN: usize = 23;
const LOGGING_MSG_MAX_LEN: usize = 4000;


pub(crate) struct CappedTag(CString);
impl CappedTag {
    pub fn new(tag: &[u8]) -> io::Result<Self> {
        let tag = if tag.len() > LOGGING_TAG_MAX_LEN {
            CString::new(
                tag.iter()
                    .take(LOGGING_TAG_MAX_LEN - 2)
                    .chain(b"..\0".iter())
                    .copied()
                    .collect::<Vec<_>>(),
            )
        } else {
            CString::new(tag.to_vec())
        }?;
        Ok(Self(tag))
    }
}

impl Deref for CappedTag {
    type Target = CStr;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


/// A writer that logs to the OpenHarmony NDK's `OH_LOG_Print` function.
/// A `inner` [BufWriter] is used to buffer the log messages and speed up frequent logging.
///
/// [BufWriter]: std::io::BufWriter
pub struct OHOSWriter<'a> {
    inner: BufWriter<HiLogWriter<'a>>,
}

impl<'a> OHOSWriter<'a> {
    pub fn new(level: &Level, domain: u16, tag: &'a CappedTag) -> Self {
        let w = HiLogWriter {
            level: match *level {
                Level::WARN => LogLevel::LOG_WARN,
                Level::INFO => LogLevel::LOG_INFO,
                Level::DEBUG => LogLevel::LOG_DEBUG,
                Level::ERROR => LogLevel::LOG_ERROR,
                Level::TRACE => LogLevel::LOG_DEBUG,
            },
            domain: 0,
            tag,
        };
        let inner = BufWriter::with_capacity(LOGGING_MSG_MAX_LEN, w);
        Self { inner }
    }
}

impl<'a> io::Write for OHOSWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

struct HiLogWriter<'a> {
    level: LogLevel,
    domain: u16,
    tag: &'a CappedTag,
}

impl<'a> HiLogWriter<'a> {
    fn log(&self, msg: &[u8]) -> io::Result<()> {
        let msg = CString::new(msg.to_vec())?;
        hilog_log(self.level, self.domain, self.tag, &msg);
        Ok(())
    }
}

impl<'a> io::Write for HiLogWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let written = buf.len().min(LOGGING_MSG_MAX_LEN);
        self.log(&buf[..written])?;
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}