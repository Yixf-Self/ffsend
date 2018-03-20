extern crate pbr;

use std::io::Stdout;
use std::time::Duration;

use ffsend_api::reader::ProgressReporter;
use self::pbr::{
    ProgressBar as Pbr,
    Units,
};

/// The refresh rate of the progress bar, in milliseconds.
const PROGRESS_BAR_FPS_MILLIS: u64 = 500;

/// A progress bar reporter.
pub struct ProgressBar<'a> {
    bar: Option<Pbr<Stdout>>,
    msg_progress: &'a str,
    msg_finish: &'a str,
}

impl<'a> ProgressBar<'a> {
    /// Construct a new progress bar, with the given messages.
    pub fn new(msg_progress: &'a str, msg_finish: &'a str) -> ProgressBar<'a> {
        Self {
            bar: None,
            msg_progress,
            msg_finish,
        }
    }

    /// Construct a new progress bar for uploading.
    pub fn new_upload() -> ProgressBar<'a> {
        Self::new("Encrypt & Upload ", "Upload complete")
    }

    /// Construct a new progress bar for downloading.
    pub fn new_download() -> ProgressBar<'a> {
        Self::new("Download & Decrypt ", "Download complete")
    }
}

impl<'a> ProgressReporter for ProgressBar<'a> {
    /// Start the progress with the given total.
    fn start(&mut self, total: u64) {
        // Initialize the progress bar
        let mut bar = Pbr::new(total);
        bar.set_max_refresh_rate(
            Some(Duration::from_millis(PROGRESS_BAR_FPS_MILLIS))
        );
        bar.set_units(Units::Bytes);
        bar.message(self.msg_progress);

        self.bar = Some(bar);
    }

    /// A progress update.
    fn progress(&mut self, progress: u64) {
        self.bar.as_mut()
            .expect("progress bar not yet instantiated, cannot set progress")
            .set(progress);
    }

    /// Finish the progress.
    fn finish(&mut self) {
        self.bar.as_mut()
            .expect("progress bar not yet instantiated")
            .finish_print(self.msg_finish);
    }
}