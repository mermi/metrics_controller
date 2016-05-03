use metrics_worker::MetricsWorker;
use events::Events;
use log::LogLevelFilter;
use logger::MetricsLoggerFactory;
use logger::MetricsLogger;
use std::sync::{Arc, Mutex};

#[allow(non_upper_case_globals)]
// Shortcut to MetricsLoggerFactory function that gets the logger instance.
const logger: fn() -> &'static MetricsLogger = MetricsLoggerFactory::get_logger;

pub struct EventInfo {
    pub locale: String,
    pub os: String,
    pub os_version: String,
    pub device: String,
    pub arch: String,
    pub app_name: String,
    pub app_version: String,
    pub app_update_channel: String,
    pub app_build_id: String,
    pub app_platform: String
}

impl EventInfo {
    pub fn new(locale: String, os: String, os_version: String, device: String, app_name: String,
               app_version: String, app_update_channel: String,
               app_build_id: String, app_platform: String,
               arch: String) -> EventInfo {

        EventInfo {
            locale: locale,
            os: os,
            os_version: os_version,
            device: device,
            app_name: app_name,
            app_version: app_version,
            app_update_channel: app_update_channel,
            app_build_id: app_build_id,
            app_platform: app_platform,
            arch: arch
        }
    }

    pub fn clone(&self) -> EventInfo {
        EventInfo {
            locale: self.locale.clone(),
            os: self.os.clone(),
            os_version: self.os_version.clone(),
            device: self.device.clone(),
            app_name: self.app_name.clone(),
            app_version: self.app_version.clone(),
            app_update_channel: self.app_update_channel.clone(),
            app_build_id: self.app_build_id.clone(),
            app_platform: self.app_platform.clone(),
            arch: self.arch.clone()
        }
    }
}

/// The metrics controller for the CD Metrics Library
pub struct MetricsController {
    #[allow(dead_code)] // Issue #33 -- Will go away with subsequent commits.
    ev: Arc<Mutex<Events>>,
    mw: MetricsWorker

}

impl MetricsController {

    //  Note: The following code example produces an 'unused variable' warning
    //        so it is being ignored for the purpose of running tests.

    /// Constructs a new `MetricsController`. Caller passes information
    /// about their application and environment and also whether the controller
    /// should be active (should be inactive, for example, if the user has
    /// opted-out of metrics collection).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use metrics_controller::controller::MetricsController;
    /// let mc = MetricsController::new(
    ///     true,
    ///     "foxbox".to_string(),
    ///     "1.0".to_string(),
    ///     "beta".to_string(),
    ///     "20160522".to_string(),
    ///     "rust".to_string(),
    ///     "en-us".to_string(),
    ///     "RPi2".to_string(),
    ///     "arm".to_string());
    /// ```
    pub fn new(app_name: String, app_version: String,
               app_update_channel: String, app_build_id: String,
               app_platform: String, locale: String,
               device: String, arch: String, os: String, os_version: String) -> MetricsController {
        logger().log(LogLevelFilter::Info, "Creating Controller");
        let event_info = EventInfo::new(
                    locale,
                    device,
                    app_name,
                    app_version,
                    app_update_channel,
                    app_build_id,
                    app_platform,
                    arch,
                    os,
                    os_version);
        let events = Arc::new(Mutex::new(Events::new(event_info)));

        MetricsController {
            ev: events.clone(),
            mw: MetricsWorker::new(events)
        }
    }

    /// This function is called to start the metrics service.  It also starts the
    /// worker thread needed to operate the metrics service.  The worker thread
    /// is responsible for periodically: persisting the histogram data and
    /// transmitting it to the telemetry server.
    pub fn start_metrics(&mut self) -> bool {

        //Data needs to be read from disk here.  Let's assume that the controller
        //owns the histogram data for now.
        // Needs to call persistence module to read the data file.
        // Call config.init()
        // Call persistence.read() and populate histograms in memory in controller.
        // histograms in separate structs in separate files.  Controller maintains
        // a refernce to the in memory histograms.  Worker thread also needs it.
        // We would prefer to use a singleton pattern.
        //MetricsWorker::new();
        true
    }

    /// Stops the metrics service and deletes metrics data that has been collected
    /// but not sent to the server.
    pub fn stop_collecting(&mut self) {
        // TODO:  Eventually, this API will need to also delete the Histograms
        // from memory and delete the ones on disk.
        self.mw.quit();
    }
}

// Create a MetricsController with predefined values
// for unit testing.
#[cfg(test)]
fn create_metrics_controller() -> MetricsController {
    MetricsController::new(
        "app".to_string(),
        "1.0".to_string(),
        "default".to_string(),
        "20160305".to_string(),
        "rust".to_string(),
        "en-us".to_string(),
        "linux".to_string(),
        "1.2.3".to_string(),
        "raspberry-pi".to_string(),
        "arm".to_string()
    )
}
