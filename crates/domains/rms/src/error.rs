use cores::app_error::AppError;

pub enum RmsError {
    ReservationNotFound,
    RoomNotAvailable,
    InvalidReservationDates,
    OverlappingReservation,
    PaymentRequired,
    ReservationAlreadyCancelled,
}

impl From<RmsError> for AppError{
    fn from(err: RmsError) -> Self {
        match err {
            RmsError::InvalidReservationDates => AppError::InvalidInput("invalid reservation".into()),
            RmsError::OverlappingReservation => AppError::InvalidInput("Overlapping reservation".into()),
            RmsError::PaymentRequired => AppError::ConfigMissing("Payment required".into()),
            RmsError::ReservationAlreadyCancelled => AppError::InvalidInput("Reservation already cancelled".into()),
            RmsError::ReservationNotFound => AppError::ConfigMissing("Reservation Not found".into()),
            RmsError::RoomNotAvailable => AppError::ConfigMissing("Room not available".into()),
        }
    }
}