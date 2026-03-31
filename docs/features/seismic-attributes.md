# Seismic Attributes Reference

## Overview

The `seisly_attributes` crate provides 20 seismic attributes for reservoir characterization, fault detection, and stratigraphic analysis.

**Categories:**
- **Amplitude Attributes (10):** Energy and strength measures
- **Frequency Attributes (10):** Spectral and instantaneous measures

## Installation

```bash
cargo add seisly_attributes
```

## Quick Start

```rust
use seisly_attributes::amplitude::{RmsAmplitude, MeanAmplitude};
use seisly_attributes::frequency::{InstantaneousFrequency, DominantFrequency};

let trace = vec![/* seismic trace */];

// Amplitude attributes
let rms = RmsAmplitude.compute(&trace, 50)?;
let mean = MeanAmplitude.compute(&trace, 50)?;

// Frequency attributes
let inst_freq = InstantaneousFrequency.compute(&trace, 0.004)?;
let dom_freq = DominantFrequency.compute(&trace, 0.004)?;
```

---

## Amplitude Attributes

### 1. RMS Amplitude

**Root Mean Square amplitude - measure of signal energy**

**Formula:**
```
RMS = sqrt(Σ(x_i²) / n)
```

**Parameters:**
- `window_size`: Window length in samples

**Use Cases:**
- Bright spot detection
- Gas chimney identification
- Reservoir energy mapping

**Example:**
```rust
use seisly_attributes::amplitude::RmsAmplitude;

let attr = RmsAmplitude;
let result = attr.compute(trace, window_size=50)?;
```

**Interpretation:**
- High RMS: High energy, potential hydrocarbons
- Low RMS: Low energy, shale background

---

### 2. Mean Amplitude

**Average amplitude value**

**Formula:**
```
Mean = Σ(x_i) / n
```

**Parameters:**
- `window_size`: Window length in samples

**Use Cases:**
- Average amplitude analysis
- Background trend removal
- Polarity analysis

**Example:**
```rust
use seisly_attributes::amplitude::MeanAmplitude;

let attr = MeanAmplitude;
let result = attr.compute(trace, window_size=50)?;
```

**Interpretation:**
- Positive mean: Peak-dominated
- Negative mean: Trough-dominated

---

### 3. Maximum Amplitude

**Peak amplitude value**

**Formula:**
```
Max = max(x_i)
```

**Parameters:**
- `window_size`: Window length in samples

**Use Cases:**
- Peak amplitude mapping
- Bright spot detection
- Anomaly identification

**Example:**
```rust
use seisly_attributes::amplitude::MaxAmplitude;

let attr = MaxAmplitude;
let result = attr.compute(trace, window_size=50)?;
```

**Interpretation:**
- High max: Strong reflector
- Useful for identifying single strong events

---

### 4. Minimum Amplitude

**Lowest amplitude value**

**Formula:**
```
Min = min(x_i)
```

**Parameters:**
- `window_size`: Window length in samples

**Use Cases:**
- Trough amplitude mapping
- Low impedance contrast detection

**Example:**
```rust
use seisly_attributes::amplitude::MinAmplitude;

let attr = MinAmplitude;
let result = attr.compute(trace, window_size=50)?;
```

**Interpretation:**
- Low min: Strong trough
- Complementary to Max Amplitude

---

### 5. Standard Deviation

**Measure of amplitude variability**

**Formula:**
```
StdDev = sqrt(Σ(x_i - mean)² / n)
```

**Parameters:**
- `window_size`: Window length in samples

**Use Cases:**
- Heterogeneity analysis
- Fault detection
- Fracture identification

**Example:**
```rust
use seisly_attributes::amplitude::StdDeviation;

let attr = StdDeviation;
let result = attr.compute(trace, window_size=50)?;
```

**Interpretation:**
- High StdDev: Heterogeneous, possible faults
- Low StdDev: Homogeneous layer

---

### 6. Energy

**Total signal energy**

**Formula:**
```
Energy = Σ(x_i²)
```

**Parameters:**
- `window_size`: Window length in samples

**Use Cases:**
- Hydrocarbon indicator
- Reservoir quality mapping
- Seismic facies analysis

**Example:**
```rust
use seisly_attributes::amplitude::Energy;

let attr = Energy;
let result = attr.compute(trace, window_size=50)?;
```

**Interpretation:**
- High energy: Strong reflectors, potential HC
- Low energy: Shale, background

---

### 7. Absolute Sum

**Sum of absolute amplitudes**

**Formula:**
```
AbsSum = Σ|x_i|
```

**Parameters:**
- `window_size`: Window length in samples

**Use Cases:**
- Amplitude strength measure
- Comparative analysis

**Example:**
```rust
use seisly_attributes::amplitude::AbsoluteSum;

let attr = AbsoluteSum;
let result = attr.compute(trace, window_size=50)?;
```

**Interpretation:**
- Proportional to reflectivity strength

---

### 8. Peak-to-Peak

**Difference between max and min**

**Formula:**
```
P2P = max(x_i) - min(x_i)
```

**Parameters:**
- `window_size`: Window length in samples

**Use Cases:**
- Amplitude variation measure
- Tuning effect analysis

**Example:**
```rust
use seisly_attributes::amplitude::PeakToPeak;

let attr = PeakToPeak;
let result = attr.compute(trace, window_size=50)?;
```

**Interpretation:**
- High P2P: Strong amplitude variation
- Useful for tuning detection

---

### 9. Variance

**Square of standard deviation**

**Formula:**
```
Variance = Σ(x_i - mean)² / n
```

**Parameters:**
- `window_size`: Window length in samples

**Use Cases:**
- Statistical analysis
- Coherence alternative

**Example:**
```rust
use seisly_attributes::amplitude::Variance;

let attr = Variance;
let result = attr.compute(trace, window_size=50)?;
```

**Interpretation:**
- Similar to StdDev, squared scale

---

### 10. Skewness

**Measure of amplitude distribution asymmetry**

**Formula:**
```
Skewness = Σ((x_i - mean)³) / (n × std³)
```

**Parameters:**
- `window_size`: Window length in samples

**Use Cases:**
- Wavelet asymmetry analysis
- Lithology indicator

**Example:**
```rust
use seisly_attributes::amplitude::Skewness;

let attr = Skewness;
let result = attr.compute(trace, window_size=50)?;
```

**Interpretation:**
- Positive skew: Peak-dominated
- Negative skew: Trough-dominated
- Zero: Symmetric distribution

---

## Frequency Attributes

### 11. Instantaneous Frequency

**Time-varying frequency content**

**Formula:**
```
IF(t) = d(phase)/dt / (2π)
```

**Parameters:**
- `dt`: Sample interval in seconds

**Use Cases:**
- Thin bed detection
- Hydrocarbon indicator
- Unconformity mapping

**Example:**
```rust
use seisly_attributes::frequency::InstantaneousFrequency;

let attr = InstantaneousFrequency;
let result = attr.compute(trace, dt=0.004)?;
```

**Interpretation:**
- High IF: Thin beds, tuning
- Low IF: Thick beds
- Anomalies: Potential HC

---

### 12. Dominant Frequency

**Frequency with maximum amplitude**

**Formula:**
```
DomFreq = argmax(|FFT(x)|)
```

**Parameters:**
- `dt`: Sample interval in seconds
- `window_size`: Window length (optional)

**Use Cases:**
- Reservoir characterization
- Lithology prediction
- Attenuation analysis

**Example:**
```rust
use seisly_attributes::frequency::DominantFrequency;

let attr = DominantFrequency;
let result = attr.compute(trace, dt=0.004)?;
```

**Interpretation:**
- High dom freq: Thin beds, shallow
- Low dom freq: Thick beds, deep

---

### 13. Peak Frequency

**Frequency at spectral peak**

**Formula:**
```
PeakFreq = frequency at max amplitude spectrum
```

**Parameters:**
- `dt`: Sample interval in seconds

**Use Cases:**
- Similar to dominant frequency
- Spectral analysis

**Example:**
```rust
use seisly_attributes::frequency::PeakFrequency;

let attr = PeakFrequency;
let result = attr.compute(trace, dt=0.004)?;
```

**Interpretation:**
- Indicates main frequency content

---

### 14. Mean Frequency

**Average frequency weighted by amplitude**

**Formula:**
```
MeanFreq = Σ(f_i × A_i) / Σ(A_i)
```

**Parameters:**
- `dt`: Sample interval in seconds

**Use Cases:**
- Frequency trend analysis
- Attenuation studies

**Example:**
```rust
use seisly_attributes::frequency::MeanFrequency;

let attr = MeanFrequency;
let result = attr.compute(trace, dt=0.004)?;
```

**Interpretation:**
- Frequency centroid measure

---

### 15. Median Frequency

**Frequency dividing spectrum equally**

**Formula:**
```
MedianFreq: Σ(A_i < MedianFreq) = Σ(A_i > MedianFreq)
```

**Parameters:**
- `dt`: Sample interval in seconds

**Use Cases:**
- Robust frequency measure
- Outlier-resistant analysis

**Example:**
```rust
use seisly_attributes::frequency::MedianFrequency;

let attr = MedianFrequency;
let result = attr.compute(trace, dt=0.004)?;
```

**Interpretation:**
- Less sensitive to spectral spikes

---

### 16. Frequency Centroid

**Center of mass of spectrum**

**Formula:**
```
Centroid = Σ(f_i × A_i²) / Σ(A_i²)
```

**Parameters:**
- `dt`: Sample interval in seconds

**Use Cases:**
- Spectral balance measure
- Reservoir characterization

**Example:**
```rust
use seisly_attributes::frequency::FrequencyCentroid;

let attr = FrequencyCentroid;
let result = attr.compute(trace, dt=0.004)?;
```

**Interpretation:**
- Similar to mean frequency, energy-weighted

---

### 17. Frequency RMS

**RMS of frequency spectrum**

**Formula:**
```
FreqRMS = sqrt(Σ(f_i² × A_i) / Σ(A_i))
```

**Parameters:**
- `dt`: Sample interval in seconds

**Use Cases:**
- Frequency energy measure
- Spectral analysis

**Example:**
```rust
use seisly_attributes::frequency::FrequencyRms;

let attr = FrequencyRms;
let result = attr.compute(trace, dt=0.004)?;
```

**Interpretation:**
- High FreqRMS: Broad spectrum

---

### 18. Frequency Skewness

**Asymmetry of frequency spectrum**

**Formula:**
```
FreqSkew = Σ((f_i - mean_f)³ × A_i) / (n × std_f³)
```

**Parameters:**
- `dt`: Sample interval in seconds

**Use Cases:**
- Spectral shape analysis
- Absorption indicator

**Example:**
```rust
use seisly_attributes::frequency::FrequencySkewness;

let attr = FrequencySkewness;
let result = attr.compute(trace, dt=0.004)?;
```

**Interpretation:**
- Positive: High-frequency enrichment
- Negative: Low-frequency enrichment (absorption)

---

### 19. Frequency Kurtosis

**Peakedness of frequency spectrum**

**Formula:**
```
FreqKurt = Σ((f_i - mean_f)⁴ × A_i) / (n × std_f⁴) - 3
```

**Parameters:**
- `dt`: Sample interval in seconds

**Use Cases:**
- Spectral peakedness
- Narrowband detection

**Example:**
```rust
use seisly_attributes::frequency::FrequencyKurtosis;

let attr = FrequencyKurtosis;
let result = attr.compute(trace, dt=0.004)?;
```

**Interpretation:**
- High kurtosis: Narrowband (tuning)
- Low kurtosis: Broadband

---

### 20. Narrowband Energy

**Energy in narrow frequency band**

**Formula:**
```
NBE = Σ(A_i) for f in [f_center - Δf, f_center + Δf]
```

**Parameters:**
- `dt`: Sample interval in seconds
- `center_freq`: Center frequency in Hz
- `bandwidth`: Band width in Hz

**Use Cases:**
- Tuning effect analysis
- Thin bed detection
- Spectral decomposition

**Example:**
```rust
use seisly_attributes::frequency::NarrowbandEnergy;

let attr = NarrowbandEnergy::new(center_freq=30.0, bandwidth=5.0);
let result = attr.compute(trace, dt=0.004)?;
```

**Interpretation:**
- High NBE: Strong energy at target frequency

---

## Batch Processing

### Computing Multiple Attributes

```rust
use seisly_attributes::{AttributeSet, AttributeBuilder};

let mut attrs = AttributeSet::new();
attrs
    .add_amplitude("rms", RmsAmplitude, 50)?
    .add_amplitude("mean", MeanAmplitude, 50)?
    .add_frequency("inst_freq", InstantaneousFrequency, 0.004)?
    .add_frequency("dom_freq", DominantFrequency, 0.004)?;

let results = attrs.compute_all(trace)?;

for (name, value) in results {
    println!("{}: {:.4}", name, value);
}
```

### Volume Processing

```rust
use seisly_attributes::volume::VolumeAttributes;

let vol_attrs = VolumeAttributes::new()
    .add_attribute(RmsAmplitude, 50)
    .add_attribute(InstantaneousFrequency, 0.004);

let result = vol_attrs.compute_volume(&seismic_volume)?;
```

## Python API

```python
from seisly.attributes import SeismicAttributes

attrs = SeismicAttributes()

# Amplitude attributes
rms = attrs.rms_amplitude(trace, window_size=50)
mean = attrs.mean_amplitude(trace, window_size=50)
std_dev = attrs.standard_deviation(trace, window_size=50)
energy = attrs.energy(trace, window_size=50)

# Frequency attributes
inst_freq = attrs.instantaneous_frequency(trace, dt=0.004)
dom_freq = attrs.dominant_frequency(trace, dt=0.004)
peak_freq = attrs.peak_frequency(trace, dt=0.004)

# Batch computation
results = attrs.compute_all(trace, [
    'rms_amplitude',
    'mean_amplitude',
    'instantaneous_frequency',
    'dominant_frequency'
])
```

## Performance Tips

1. **Window Size:** Choose appropriate window for your objective
   - Thin beds: Small window (10-30 samples)
   - Thick sequences: Large window (50-100 samples)

2. **Multi-threading:** Enable parallel computation
   ```rust
   let attrs = VolumeAttributes::new().with_threads(4);
   ```

3. **Memory:** Process in chunks for large volumes
   ```rust
   for chunk in volume.chunks() {
       attrs.compute(&chunk)?;
   }
   ```

## Attribute Selection Guide

| Objective | Recommended Attributes |
|-----------|----------------------|
| Bright Spot Detection | RMS, Energy, Max Amplitude |
| Thin Bed Detection | Instantaneous Frequency, Dominant Frequency |
| Fault Detection | Std Deviation, Variance |
| HC Indicator | RMS, Energy, Instantaneous Frequency, Low Freq |
| Lithology | Mean Amplitude, Dominant Frequency, Skewness |
| Tuning Effect | Peak-to-Peak, Frequency Kurtosis, Narrowband Energy |

## References

- Taner, M. T., et al. (1979). Complex seismic trace analysis.
- Chopra, S., & Marfurt, K. J. (2007). Seismic attributes for prospect mapping.
- Brown, A. R. (2004). Interpretation of three-dimensional seismic data.
