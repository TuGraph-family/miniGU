use std::time::Duration;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryMetrics {
    pub(crate) parsing_time: Duration,
    pub(crate) binding_time: Duration,
    pub(crate) planning_time: Duration,
    pub(crate) execution_time: Duration,
}

impl QueryMetrics {
    /// Returns the time taken to parse the query.
    #[inline]
    pub fn parsing_time(&self) -> Duration {
        self.parsing_time
    }

    /// Returns the time taken to bind the query.
    #[inline]
    pub fn binding_time(&self) -> Duration {
        self.binding_time
    }

    /// Returns the time taken to plan the query.
    #[inline]
    pub fn planning_time(&self) -> Duration {
        self.planning_time
    }

    /// Returns the time taken to compile the query.
    ///
    /// This equals to `parsing_time + binding_time + planning_time`.
    #[inline]
    pub fn compiling_time(&self) -> Duration {
        self.parsing_time + self.binding_time + self.planning_time
    }

    /// Returns the time taken to execute the query plan.
    #[inline]
    pub fn execution_time(&self) -> Duration {
        self.execution_time
    }

    /// Returns the total time taken to execute the query.
    #[inline]
    pub fn total_time(&self) -> Duration {
        self.parsing_time + self.binding_time + self.planning_time + self.execution_time
    }
}
