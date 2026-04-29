use crate::proto::AppTime;

#[derive(Debug, Clone)]
pub struct RustTime {
    pub day_length_minutes: f32,
    pub time_scale: f32,
    pub sunrise: f32,
    pub sunset: f32,
    pub time: f32,
}

impl From<AppTime> for RustTime {
    fn from(app_time: AppTime) -> Self {
        Self {
            day_length_minutes: app_time.day_length_minutes,
            time_scale: app_time.time_scale,
            sunrise: app_time.sunrise,
            sunset: app_time.sunset,
            time: app_time.time,
        }
    }
}
