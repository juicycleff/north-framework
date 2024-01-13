use crate::registry::service_status_constants::ServiceStatus;
use chrono::Local;

/// [ServiceInstanceState] contains state of every service node
#[derive(Clone)]
pub struct ServiceInstanceState {
    /// service status
    status: ServiceStatus,

    /// total api request count
    total_requests: i64,

    /// total active request count
    active_requests_count: i64,

    /// weight of the service
    weight: i64,

    /// API avg response time is milliseconds
    response_time_avg: i64,

    /// API max response time is milliseconds
    response_time_max: i64,

    /// active request count timeout in milliseconds
    active_requests_count_timeout: i16,

    /// last active requests count change timestamp in milliseconds
    last_active_requests_count_change_timestamp: i64,

    /// first connection timestamp in milliseconds
    first_connection_timestamp: i64,

    /// last connection failed timestamp in milliseconds
    last_connection_failed_timestamp: Option<i64>,

    /// last connection failed message
    last_connection_failed_message: String,

    /// count of requests failures
    failure_counts: i64,

    /// fixed weight
    fixed_weight: bool,
}

impl Default for ServiceInstanceState {
    fn default() -> Self {
        ServiceInstanceState {
            status: Default::default(),
            total_requests: 0,
            active_requests_count: 0,
            weight: 0,
            response_time_avg: 0,
            response_time_max: 0,
            active_requests_count_timeout: 0,
            last_active_requests_count_change_timestamp: 0,
            first_connection_timestamp: 0,
            last_connection_failed_timestamp: None,
            last_connection_failed_message: "".to_string(),
            failure_counts: 0,
            fixed_weight: false,
        }
    }
}

impl ServiceInstanceState {
    pub fn new(fixed_weight: Option<bool>) -> Self {
        ServiceInstanceState {
            status: Default::default(),
            total_requests: 0,
            active_requests_count: 0,
            weight: 0,
            response_time_avg: 0,
            response_time_max: 0,
            active_requests_count_timeout: 0,
            last_active_requests_count_change_timestamp: 0,
            first_connection_timestamp: 0,
            last_connection_failed_timestamp: None,
            last_connection_failed_message: "".to_string(),
            failure_counts: 0,
            fixed_weight: fixed_weight.unwrap_or(false),
        }
    }

    pub fn get_active_requests_count(&mut self, current_time: Option<i64>) -> i64 {
        let mut _current_time = current_time.unwrap_or_else(|| Local::now().timestamp_millis());
        let count = self.active_requests_count;
        if count == 0 {
            0
        } else if _current_time - self.last_active_requests_count_change_timestamp
            > (self.active_requests_count_timeout * 1000) as i64
            || count < 0
        {
            self.active_requests_count = 0;
            self.active_requests_count
        } else {
            count
        }
    }

    pub fn is_healthy(&self) -> bool {
        !matches!(self.status, ServiceStatus::Critical)
    }

    pub fn increment_failure_counts(&mut self) -> i64 {
        self.failure_counts += 1;
        self.failure_counts
    }

    pub fn increment_request_counts(&mut self) -> i64 {
        self.total_requests += 1;
        self.total_requests
    }

    pub fn increment_active_requests(&mut self) -> i64 {
        self.last_active_requests_count_change_timestamp = Local::now().timestamp_millis();
        self.active_requests_count += 1;
        self.active_requests_count
    }

    pub fn decrement_active_requests(&mut self) -> i64 {
        if self.active_requests_count < 1 {
            return 0;
        }

        self.last_active_requests_count_change_timestamp = Local::now().timestamp_millis();
        self.active_requests_count -= 1;
        self.active_requests_count
    }

    pub fn set_connection_failed_time(&mut self, message: Option<String>) {
        self.last_connection_failed_timestamp = Some(Local::now().timestamp_millis());
        self.last_connection_failed_message = message.unwrap_or_default();
        self.status = ServiceStatus::Critical;
    }

    pub fn set_first_connection_time(&mut self) {
        self.first_connection_timestamp = Local::now().timestamp_millis();
    }

    pub fn set_response_time(&mut self, time: i64) {
        if !self.fixed_weight {
            self.weight = time - self.response_time_avg;
        }
        self.response_time_avg =
            (self.response_time_avg * (self.total_requests - 1) + time) / self.total_requests;
        self.response_time_max = std::cmp::max(self.response_time_max, time);
    }
}
