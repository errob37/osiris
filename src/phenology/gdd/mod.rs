/// The cutoff method refers to the manner in which the degree-day calculation area will
/// be modified in relation to the upper threshold.
///
/// Reference: University pf California Agriculture & Natural Resources
/// `<https://ipm.ucanr.edu/WEATHER/ddconcepts.html>`
pub enum CutoffMethod {
    /// The horizontal cutoff method assumes that development continues at a constant rate
    /// at temperatures in excess of the upper threshold
    Horizontal,
    /// The intermediate cutoff assumes that development slows, but does not stop,
    /// at temperatures above the upper threshold.
    Intermediate,
    /// The vertical cutoff method assumes that no development occurs when a
    /// temperature is above the upper threshold
    Vertical,
    ///No cutoff method assumes that development continues at a linear rate
    /// at temperatures in excess of the upper threshold
    None,
}

/// Although it is simple to calculate the degree-days accumulated at a constant temperature
/// in the laboratory, calculating degree-days for the daily temperature fluctuations that occur
/// in nature is more difficult. Several methods are used to estimate degree-days through the use
/// of daily minimum and maximum temperatures.
///
/// All are approximations of the actual number of degree-days accumulated for a given
/// set of daily temperatures and developmental thresholds, and therefore do not provide
/// the exact degree-day values.
///
/// However, most are adequate considering the accuracy of weather instruments used and
/// the precision required for crop management decisions.
pub enum CalculationMethod {
    /// This technique uses a day's minimum and maximum temperatures to produce a sine curve
    /// over a 24-hour period, and then estimates degree-days for that day by calculating the
    /// area above the threshold and below the curve. This method assumes the temperature
    /// curve is symmetrical around the maximum temperature
    SingleSine,
    /// This method fits a sine curve from the minimum temperature of the day to the maximum
    /// temperature of the day and then fits a separate sine curve from the maximum temperature
    /// of the day to the minimum temperature of the next day.
    /// Degree-days for the day are the sum of the degree-days for the two half-days
    DoubleSine,
    /// The method draws a straight line between a day's minimum temperature and
    /// maximum temperature, assumes the next day's minimum temperature is the same,
    /// and draws another line to that point, forming two sides of a triangle.
    /// This method assumes the temperature curve is symmetrical around the maximum temperature.
    /// Degree-days are estimated by calculating the area within the triangle and between the thresholds.
    SingleTriangulation,
    /// Using two 12-hour or half-day calculations, the double triangle method draws a straight
    /// line between a daily minimum and maximum temperature, and another line vertically through
    /// the maximum temperature, forming two sides of a triangle.
    /// Degree-days are estimated by calculating the area within the triangle and between the thresholds.
    /// The second 12-hour period uses the same configuration with the minimum temperature of the following day.
    /// Degree-days for the day are the sum of the degree-days for the two half-days
    DoubleTriangulation,
}

pub enum TemperatureUnit {
    Fahrenheit,
    Celsius
}

pub struct Temperature {
    unit: TemperatureUnit,
    value: f64
}
