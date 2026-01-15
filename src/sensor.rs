//! Sensor Input Abstraction
//!
//! Platform-agnostic sensor input types for mobile AI orchestration.
//! Learned from neurophone's sensor processing patterns.
//!
//! # Design Goals
//!
//! - **Platform agnostic**: Works on Android, iOS, embedded, or desktop
//! - **Zero-copy friendly**: Use references where possible
//! - **Feature extraction**: Convert raw readings to neural-friendly inputs
//!
//! # Usage
//!
//! ```rust,ignore
//! use mobile_ai_orchestrator::sensor::{SensorReading, SensorType};
//!
//! let reading = SensorReading::new(
//!     SensorType::Accelerometer,
//!     vec![0.1, -9.8, 0.3],  // x, y, z
//! );
//!
//! // Feed to reservoir/snn
//! let features = reading.to_features();
//! ```

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Sensor types supported by the orchestrator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SensorType {
    /// 3-axis accelerometer (x, y, z in m/s^2)
    Accelerometer,
    /// 3-axis gyroscope (x, y, z in rad/s)
    Gyroscope,
    /// 3-axis magnetometer (x, y, z in uT)
    Magnetometer,
    /// Ambient light sensor (lux)
    Light,
    /// Proximity sensor (cm or binary 0/1)
    Proximity,
    /// Barometer/altimeter (hPa)
    Barometer,
    /// GPS location (lat, lon, accuracy_m)
    Gps,
    /// Audio amplitude or feature
    Audio,
    /// Touch coordinates (x, y normalized 0-1)
    Touch,
    /// Custom/user-defined sensor
    Custom(u8),
}

impl SensorType {
    /// Expected number of values for this sensor type
    pub const fn dimensions(&self) -> usize {
        match self {
            SensorType::Accelerometer => 3,
            SensorType::Gyroscope => 3,
            SensorType::Magnetometer => 3,
            SensorType::Light => 1,
            SensorType::Proximity => 1,
            SensorType::Barometer => 1,
            SensorType::Gps => 3,
            SensorType::Audio => 1,
            SensorType::Touch => 2,
            SensorType::Custom(_) => 1,
        }
    }

    /// Human-readable name
    pub const fn name(&self) -> &'static str {
        match self {
            SensorType::Accelerometer => "accelerometer",
            SensorType::Gyroscope => "gyroscope",
            SensorType::Magnetometer => "magnetometer",
            SensorType::Light => "light",
            SensorType::Proximity => "proximity",
            SensorType::Barometer => "barometer",
            SensorType::Gps => "gps",
            SensorType::Audio => "audio",
            SensorType::Touch => "touch",
            SensorType::Custom(_) => "custom",
        }
    }
}

/// Accuracy/reliability of sensor reading
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum SensorAccuracy {
    /// Sensor data unreliable (e.g., uncalibrated)
    Unreliable,
    /// Low accuracy
    Low,
    /// Medium accuracy
    #[default]
    Medium,
    /// High accuracy (calibrated)
    High,
}

/// A single sensor reading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorReading {
    /// Type of sensor
    pub sensor_type: SensorType,
    /// Timestamp in milliseconds since epoch
    pub timestamp_ms: u64,
    /// Raw sensor values
    pub values: Vec<f32>,
    /// Accuracy of reading
    pub accuracy: SensorAccuracy,
}

impl SensorReading {
    /// Create a new sensor reading with current timestamp
    pub fn new(sensor_type: SensorType, values: Vec<f32>) -> Self {
        Self {
            sensor_type,
            timestamp_ms: current_timestamp_ms(),
            values,
            accuracy: SensorAccuracy::Medium,
        }
    }

    /// Create with explicit timestamp
    pub fn with_timestamp(sensor_type: SensorType, values: Vec<f32>, timestamp_ms: u64) -> Self {
        Self {
            sensor_type,
            timestamp_ms,
            values,
            accuracy: SensorAccuracy::Medium,
        }
    }

    /// Set accuracy level
    pub fn with_accuracy(mut self, accuracy: SensorAccuracy) -> Self {
        self.accuracy = accuracy;
        self
    }

    /// Convert to feature vector for neural input
    ///
    /// Normalizes values based on typical sensor ranges:
    /// - Accelerometer: /-20 m/s^2 -> [-1, 1]
    /// - Gyroscope: /-10 rad/s -> [-1, 1]
    /// - Light: 0-10000 lux -> [0, 1]
    /// - etc.
    pub fn to_features(&self) -> Vec<f32> {
        let scale = match self.sensor_type {
            SensorType::Accelerometer => 20.0,  // /-20 m/s^2
            SensorType::Gyroscope => 10.0,      // /-10 rad/s
            SensorType::Magnetometer => 100.0,  // /-100 uT
            SensorType::Light => 10000.0,       // 0-10000 lux
            SensorType::Proximity => 10.0,      // 0-10 cm
            SensorType::Barometer => 200.0,     // ~900-1100 hPa, center at 1000
            SensorType::Gps => 180.0,           // lat/lon degrees
            SensorType::Audio => 1.0,           // assume pre-normalized
            SensorType::Touch => 1.0,           // already 0-1
            SensorType::Custom(_) => 1.0,       // assume pre-normalized
        };

        self.values.iter().map(|v| v / scale).collect()
    }

    /// Get magnitude of vector sensors (accelerometer, gyroscope, etc.)
    pub fn magnitude(&self) -> f32 {
        self.values
            .iter()
            .map(|v| v * v)
            .sum::<f32>()
            .sqrt()
    }
}

/// Buffer for collecting sensor readings over time
#[derive(Debug, Clone)]
pub struct SensorBuffer {
    readings: Vec<SensorReading>,
    max_size: usize,
}

impl SensorBuffer {
    /// Create a new buffer with maximum size
    pub fn new(max_size: usize) -> Self {
        Self {
            readings: Vec::with_capacity(max_size),
            max_size,
        }
    }

    /// Add a reading (drops oldest if full)
    pub fn push(&mut self, reading: SensorReading) {
        if self.readings.len() >= self.max_size {
            self.readings.remove(0);
        }
        self.readings.push(reading);
    }

    /// Get all readings
    pub fn readings(&self) -> &[SensorReading] {
        &self.readings
    }

    /// Get readings of a specific type
    pub fn readings_of_type(&self, sensor_type: SensorType) -> Vec<&SensorReading> {
        self.readings
            .iter()
            .filter(|r| r.sensor_type == sensor_type)
            .collect()
    }

    /// Convert buffer to feature matrix (flattened)
    ///
    /// Returns a flat vector suitable for reservoir/SNN input
    pub fn to_feature_vector(&self) -> Vec<f32> {
        self.readings
            .iter()
            .flat_map(|r| r.to_features())
            .collect()
    }

    /// Clear the buffer
    pub fn clear(&mut self) {
        self.readings.clear();
    }

    /// Number of readings in buffer
    pub fn len(&self) -> usize {
        self.readings.len()
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.readings.is_empty()
    }
}

/// Get current timestamp in milliseconds
fn current_timestamp_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensor_dimensions() {
        assert_eq!(SensorType::Accelerometer.dimensions(), 3);
        assert_eq!(SensorType::Light.dimensions(), 1);
        assert_eq!(SensorType::Touch.dimensions(), 2);
    }

    #[test]
    fn test_reading_to_features() {
        let reading = SensorReading::new(
            SensorType::Accelerometer,
            vec![0.0, -9.8, 0.0],
        );
        let features = reading.to_features();
        assert_eq!(features.len(), 3);
        // -9.8 / 20.0 = -0.49
        assert!((features[1] - (-0.49)).abs() < 0.01);
    }

    #[test]
    fn test_magnitude() {
        let reading = SensorReading::new(
            SensorType::Accelerometer,
            vec![3.0, 4.0, 0.0],
        );
        assert!((reading.magnitude() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_buffer() {
        let mut buffer = SensorBuffer::new(3);
        buffer.push(SensorReading::new(SensorType::Light, vec![100.0]));
        buffer.push(SensorReading::new(SensorType::Light, vec![200.0]));
        buffer.push(SensorReading::new(SensorType::Light, vec![300.0]));
        buffer.push(SensorReading::new(SensorType::Light, vec![400.0])); // drops first

        assert_eq!(buffer.len(), 3);
        assert_eq!(buffer.readings()[0].values[0], 200.0);
    }
}
