# Fee Insights Design Document

## Overview

The Fee Insights system is a computational engine that transforms raw blockchain fee data into meaningful analytical insights. It operates as a modular component within the existing Stellar Fee Tracker core service, providing rolling averages, extreme value tracking, and congestion trend detection. The system is designed to be data-source agnostic and operates on a polling-based architecture that integrates seamlessly with the existing HorizonClient infrastructure.

## Architecture

The Fee Insights system follows a layered architecture that separates data ingestion, computation, and storage concerns:

```
┌─────────────────────────────────────────────────────────────┐
│                    Fee Insights Engine                      │
├─────────────────────────────────────────────────────────────┤
│  Analytics Layer                                            │
│  ├── Rolling Average Calculator                             │
│  ├── Extremes Tracker                                       │
│  └── Congestion Trend Detector                              │
├─────────────────────────────────────────────────────────────┤
│  Data Processing Layer                                      │
│  ├── Fee Data Normalizer                                    │
│  ├── Time Window Manager                                    │
│  └── Metrics Aggregator                                     │
├─────────────────────────────────────────────────────────────┤
│  Storage Layer                                              │
│  ├── In-Memory Cache                                        │
│  └── Historical Data Store                                  │
├─────────────────────────────────────────────────────────────┤
│  Data Source Abstraction                                   │
│  └── Fee Data Provider Interface                            │
└─────────────────────────────────────────────────────────────┘
```

The system integrates with the existing service architecture through the HorizonClient and extends the current polling mechanism to include fee insights computation.

## Components and Interfaces

### Fee Insights Engine
The central orchestrator that coordinates all fee analysis operations:

```rust
pub struct FeeInsightsEngine {
    config: InsightsConfig,
    calculator: RollingAverageCalculator,
    tracker: ExtremesTracker,
    detector: CongestionDetector,
    storage: InsightsStorage,
}

impl FeeInsightsEngine {
    pub async fn process_fee_data(&mut self, data: &[FeeDataPoint]) -> Result<InsightsUpdate, InsightsError>;
    pub fn get_current_insights(&self) -> CurrentInsights;
    pub fn get_rolling_averages(&self) -> RollingAverages;
    pub fn get_extremes(&self) -> FeeExtremes;
    pub fn get_congestion_trends(&self) -> CongestionTrends;
}
```

### Fee Data Provider Interface
Abstracts data source details to ensure engine independence:

```rust
pub trait FeeDataProvider {
    async fn fetch_latest_fees(&self) -> Result<Vec<FeeDataPoint>, ProviderError>;
    fn provider_name(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct FeeDataPoint {
    pub fee_amount: u64,
    pub timestamp: DateTime<Utc>,
    pub transaction_hash: String,
    pub ledger_sequence: u64,
}
```

### Rolling Average Calculator
Computes and maintains rolling averages across multiple time windows:

```rust
pub struct RollingAverageCalculator {
    windows: HashMap<TimeWindow, CircularBuffer<FeeDataPoint>>,
    config: AverageConfig,
}

impl RollingAverageCalculator {
    pub fn add_data_point(&mut self, point: FeeDataPoint);
    pub fn calculate_averages(&self) -> HashMap<TimeWindow, AverageResult>;
    pub fn get_average_for_window(&self, window: TimeWindow) -> Option<AverageResult>;
}
```

### Extremes Tracker
Tracks minimum and maximum fee values across different time periods:

```rust
pub struct ExtremesTracker {
    current_period: ExtremePeriod,
    historical_periods: Vec<ExtremePeriod>,
    config: ExtremesConfig,
}

impl ExtremesTracker {
    pub fn update_with_fees(&mut self, fees: &[FeeDataPoint]);
    pub fn get_current_extremes(&self) -> FeeExtremes;
    pub fn get_historical_extremes(&self, period: TimePeriod) -> Vec<FeeExtremes>;
}
```

### Congestion Trend Detector
Analyzes fee patterns to identify network congestion:

```rust
pub struct CongestionDetector {
    spike_threshold_config: SpikeConfig,
    trend_analyzer: TrendAnalyzer,
    historical_spikes: VecDeque<FeeSpike>,
}

impl CongestionDetector {
    pub fn analyze_congestion(&mut self, current_fees: &[FeeDataPoint], baseline: f64) -> CongestionAnalysis;
    pub fn detect_spikes(&self, fees: &[FeeDataPoint], baseline: f64) -> Vec<FeeSpike>;
    pub fn calculate_trend_strength(&self) -> TrendStrength;
}
```

## Data Models

### Core Data Structures

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentInsights {
    pub rolling_averages: RollingAverages,
    pub extremes: FeeExtremes,
    pub congestion_trends: CongestionTrends,
    pub last_updated: DateTime<Utc>,
    pub data_quality: DataQuality,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollingAverages {
    pub short_term: AverageResult,    // 1 hour
    pub medium_term: AverageResult,   // 6 hours
    pub long_term: AverageResult,     // 24 hours
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AverageResult {
    pub value: f64,
    pub sample_count: usize,
    pub is_partial: bool,
    pub calculated_at: DateTime<Utc>,
    pub time_window: TimeWindow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeExtremes {
    pub current_min: ExtremeValue,
    pub current_max: ExtremeValue,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtremeValue {
    pub value: u64,
    pub timestamp: DateTime<Utc>,
    pub transaction_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CongestionTrends {
    pub current_trend: TrendIndicator,
    pub recent_spikes: Vec<FeeSpike>,
    pub trend_strength: TrendStrength,
    pub predicted_duration: Option<Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeSpike {
    pub peak_fee: u64,
    pub baseline_fee: f64,
    pub spike_ratio: f64,
    pub start_time: DateTime<Utc>,
    pub duration: Duration,
    pub severity: SpikeSeverity,
}
```

### Configuration Models

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsightsConfig {
    pub polling_interval: Duration,
    pub time_windows: Vec<TimeWindow>,
    pub spike_detection: SpikeConfig,
    pub storage_retention: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeWindow {
    pub name: String,
    pub duration: Duration,
    pub min_samples: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpikeConfig {
    pub threshold_multiplier: f64,
    pub minimum_spike_duration: Duration,
    pub congestion_window: Duration,
}
```

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system-essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property Reflection

After analyzing all acceptance criteria, several properties can be consolidated to eliminate redundancy:

- Properties 1.1, 1.3, and 1.5 all relate to rolling average calculation and can be combined into a comprehensive rolling average property
- Properties 2.1, 2.3, and 2.4 all relate to extremes tracking and can be combined into a comprehensive extremes tracking property  
- Properties 3.1, 3.2, 3.3, 3.4, and 3.5 all relate to congestion detection and can be combined into comprehensive congestion analysis properties
- Properties 4.1, 4.4, and 4.5 relate to data source independence and can be combined

### Core Properties

**Property 1: Rolling Average Calculation Correctness**
*For any* sequence of fee data points and any configured time window, calculating rolling averages should produce mathematically correct averages that update within the polling cycle and return current values with proper timestamps when queried
**Validates: Requirements 1.1, 1.3, 1.5**

**Property 2: Time Window Separation**
*For any* fee data input, the system should maintain completely separate rolling average calculations for each configured time window (short-term, medium-term, long-term)
**Validates: Requirements 1.2**

**Property 3: Extremes Tracking Accuracy**
*For any* set of fee data points processed during polling cycles, the system should correctly identify and record the true minimum and maximum values, update them immediately when new extremes are found, and return complete extreme data with timestamps when queried
**Validates: Requirements 2.1, 2.3, 2.4**

**Property 4: Historical Extremes Preservation**
*For any* sequence of tracking periods, the system should maintain accurate historical records of min/max values across different time periods and preserve historical data when resetting tracking windows
**Validates: Requirements 2.2, 2.5**

**Property 5: Spike Detection Accuracy**
*For any* fee data and rolling average baseline, the system should correctly detect fee spikes by comparing current fees against baselines and classify them accurately by severity and duration
**Validates: Requirements 3.1, 3.2**

**Property 6: Congestion Trend Analysis**
*For any* collection of fee spikes within time windows, the system should correctly aggregate them into congestion trend indicators, calculate accurate trend strength and predicted duration, and update indicators within the polling cycle
**Validates: Requirements 3.3, 3.4, 3.5**

**Property 7: Data Source Independence**
*For any* fee data provided through the standardized interface, the system should process it identically regardless of the original data source, maintaining calculation continuity when switching sources and operating gracefully with cached data during source failures
**Validates: Requirements 4.1, 4.4, 4.5**

## Error Handling

The Fee Insights Engine implements comprehensive error handling across all layers:

### Data Processing Errors
- **Invalid Fee Data**: Malformed or negative fee values are logged and skipped, with data quality metrics updated
- **Timestamp Issues**: Out-of-order or future timestamps are handled by sorting and validation
- **Missing Data Points**: Gaps in data are detected and marked in data quality indicators

### Calculation Errors
- **Insufficient Data**: When time windows lack minimum required samples, calculations proceed with available data and results are marked as partial
- **Numerical Overflow**: Large fee values are handled using appropriate numeric types with overflow detection
- **Division by Zero**: Baseline calculations include zero-checking with fallback to historical averages

### Storage Errors
- **Memory Pressure**: Circular buffers automatically evict oldest data when memory limits are approached
- **Persistence Failures**: In-memory operations continue even if historical storage fails, with error logging

### Integration Errors
- **Data Source Failures**: Engine continues with cached data and marks data quality as degraded
- **Configuration Errors**: Invalid configurations are validated at startup with clear error messages
- **Polling Failures**: Missed polling cycles are logged and the next cycle processes accumulated data

## Testing Strategy

The Fee Insights system employs a dual testing approach combining unit tests and property-based tests to ensure comprehensive correctness validation.

### Property-Based Testing

**Framework**: The system uses `proptest` for Rust property-based testing, configured to run a minimum of 100 iterations per property to ensure statistical confidence.

**Property Test Requirements**:
- Each correctness property must be implemented by a single property-based test
- Tests must be tagged with comments explicitly referencing the design document property: `**Feature: fee-insights, Property {number}: {property_text}**`
- Property tests verify universal behaviors across all valid inputs
- Generators create realistic fee data patterns including edge cases

**Property Test Coverage**:
- Rolling average calculations across various data distributions and time windows
- Extremes tracking with random fee sequences and boundary conditions  
- Congestion detection with synthetic spike patterns and baseline variations
- Data source independence with multiple mock provider implementations
- Error handling with invalid inputs and failure scenarios

### Unit Testing

**Scope**: Unit tests complement property tests by verifying specific examples, integration points, and concrete edge cases.

**Unit Test Coverage**:
- Specific mathematical calculations with known input/output pairs
- Configuration parsing and validation logic
- Error condition handling with specific failure scenarios
- Integration between engine components
- Time window management and data retention policies

**Test Organization**:
- Tests are co-located with source files using `.rs` files in `tests/` modules
- Each major component has dedicated test modules
- Integration tests verify end-to-end workflows
- Performance tests validate calculation timing requirements

### Test Data Generation

**Realistic Data Patterns**:
- Fee data generators create realistic Stellar network fee distributions
- Spike generators create various congestion patterns (gradual, sudden, sustained)
- Time series generators create realistic temporal patterns with gaps and bursts
- Edge case generators create boundary conditions (empty data, single points, extreme values)

**Data Quality Validation**:
- Generated data includes quality indicators and metadata
- Test scenarios cover partial data, missing timestamps, and out-of-order sequences
- Stress testing with large datasets and extended time periods

This comprehensive testing strategy ensures that the Fee Insights Engine maintains correctness across all operational scenarios while providing confidence in its analytical accuracy and reliability.