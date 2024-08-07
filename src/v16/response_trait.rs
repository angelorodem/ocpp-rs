use alloc::string::String;

use super::call::{self};
use super::call_result::{self};
use super::parse;

/// Trait to define what response should be for a given Call Payload.
pub trait Response {
    type ResponseType;
    /// Get the response for a given Call Payload.     
    /// - `unique_id` is the unique id of the call, a number string.    
    /// - `payload` is the response payload.    
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message;
}

impl Response for call::Authorize {
    type ResponseType = call_result::GenericIdTagInfo;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleEmptyResponse(
                call_result::EmptyResponses::GenericIdTagInfoResponse(payload),
            ),
        ))
    }
}

impl Response for call::BootNotification {
    type ResponseType = call_result::BootNotification;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::BootNotification(payload),
        ))
    }
}

impl Response for call::CancelReservation {
    type ResponseType = call_result::GenericStatusResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleStatusResponse(
                call_result::StatusResponses::StatusResponse(payload),
            ),
        ))
    }
}

impl Response for call::CertificateSigned {
    type ResponseType = call_result::GenericStatusResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleStatusResponse(
                call_result::StatusResponses::StatusResponse(payload),
            ),
        ))
    }
}

impl Response for call::ChangeAvailability {
    type ResponseType = call_result::GenericStatusResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleStatusResponse(
                call_result::StatusResponses::StatusResponse(payload),
            ),
        ))
    }
}

impl Response for call::ChangeConfiguration {
    type ResponseType = call_result::GenericStatusResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleStatusResponse(
                call_result::StatusResponses::StatusResponse(payload),
            ),
        ))
    }
}

impl Response for call::ClearCache {
    type ResponseType = call_result::GenericStatusResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleStatusResponse(
                call_result::StatusResponses::StatusResponse(payload),
            ),
        ))
    }
}

impl Response for call::ClearChargingProfile {
    type ResponseType = call_result::GenericStatusResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleStatusResponse(
                call_result::StatusResponses::StatusResponse(payload),
            ),
        ))
    }
}

impl Response for call::DataTransfer {
    type ResponseType = call_result::DataTransfer;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleStatusResponse(
                call_result::StatusResponses::DataTransfer(payload),
            ),
        ))
    }
}

impl Response for call::DeleteCertificate {
    type ResponseType = call_result::GenericStatusResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleStatusResponse(
                call_result::StatusResponses::StatusResponse(payload),
            ),
        ))
    }
}

impl Response for call::DiagnosticsStatusNotification {
    type ResponseType = call_result::EmptyResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleEmptyResponse(
                call_result::EmptyResponses::EmptyResponse(payload),
            ),
        ))
    }
}

impl Response for call::ExtendedTriggerMessage {
    type ResponseType = call_result::GenericStatusResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleStatusResponse(
                call_result::StatusResponses::StatusResponse(payload),
            ),
        ))
    }
}

impl Response for call::FirmwareStatusNotification {
    type ResponseType = call_result::EmptyResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleEmptyResponse(
                call_result::EmptyResponses::EmptyResponse(payload),
            ),
        ))
    }
}

impl Response for call::GetCompositeSchedule {
    type ResponseType = call_result::GetCompositeSchedule;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleStatusResponse(
                call_result::StatusResponses::GetCompositeSchedule(payload),
            ),
        ))
    }
}

impl Response for call::GetConfiguration {
    type ResponseType = call_result::GetConfiguration;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleEmptyResponse(
                call_result::EmptyResponses::GetConfiguration(payload),
            ),
        ))
    }
}

impl Response for call::GetDiagnostics {
    type ResponseType = call_result::GetDiagnostics;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleEmptyResponse(
                call_result::EmptyResponses::GetDiagnostics(payload),
            ),
        ))
    }
}

impl Response for call::GetInstalledCertificateIds {
    type ResponseType = call_result::GetInstalledCertificateIds;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleStatusResponse(
                call_result::StatusResponses::GetInstalledCertificateIds(payload),
            ),
        ))
    }
}

impl Response for call::GetLocalListVersion {
    type ResponseType = call_result::GetLocalListVersion;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::GetLocalListVersion(payload),
        ))
    }
}

impl Response for call::GetLog {
    type ResponseType = call_result::GetLog;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleStatusResponse(
                call_result::StatusResponses::GetLog(payload),
            ),
        ))
    }
}

impl Response for call::Heartbeat {
    type ResponseType = call_result::Heartbeat;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::Heartbeat(payload),
        ))
    }
}

impl Response for call::InstallCertificate {
    type ResponseType = call_result::GenericStatusResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleStatusResponse(
                call_result::StatusResponses::StatusResponse(payload),
            ),
        ))
    }
}

impl Response for call::LogStatusNotification {
    type ResponseType = call_result::EmptyResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleEmptyResponse(
                call_result::EmptyResponses::EmptyResponse(payload),
            ),
        ))
    }
}

impl Response for call::MeterValues {
    type ResponseType = call_result::EmptyResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleEmptyResponse(
                call_result::EmptyResponses::EmptyResponse(payload),
            ),
        ))
    }
}

impl Response for call::RemoteStartTransaction {
    type ResponseType = call_result::GenericStatusResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleStatusResponse(
                call_result::StatusResponses::StatusResponse(payload),
            ),
        ))
    }
}

impl Response for call::RemoteStopTransaction {
    type ResponseType = call_result::GenericStatusResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleStatusResponse(
                call_result::StatusResponses::StatusResponse(payload),
            ),
        ))
    }
}

impl Response for call::ReserveNow {
    type ResponseType = call_result::GenericStatusResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleStatusResponse(
                call_result::StatusResponses::StatusResponse(payload),
            ),
        ))
    }
}

impl Response for call::Reset {
    type ResponseType = call_result::GenericStatusResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleStatusResponse(
                call_result::StatusResponses::StatusResponse(payload),
            ),
        ))
    }
}

impl Response for call::SecurityEventNotification {
    type ResponseType = call_result::EmptyResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleEmptyResponse(
                call_result::EmptyResponses::EmptyResponse(payload),
            ),
        ))
    }
}

impl Response for call::SendLocalList {
    type ResponseType = call_result::GenericStatusResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleStatusResponse(
                call_result::StatusResponses::StatusResponse(payload),
            ),
        ))
    }
}

impl Response for call::SetChargingProfile {
    type ResponseType = call_result::GenericStatusResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleStatusResponse(
                call_result::StatusResponses::StatusResponse(payload),
            ),
        ))
    }
}

impl Response for call::SignCertificate {
    type ResponseType = call_result::GenericStatusResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleStatusResponse(
                call_result::StatusResponses::StatusResponse(payload),
            ),
        ))
    }
}

impl Response for call::SignedFirmwareStatusNotification {
    type ResponseType = call_result::EmptyResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleEmptyResponse(
                call_result::EmptyResponses::EmptyResponse(payload),
            ),
        ))
    }
}

impl Response for call::SignedUpdateFirmware {
    type ResponseType = call_result::GenericStatusResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleStatusResponse(
                call_result::StatusResponses::StatusResponse(payload),
            ),
        ))
    }
}

impl Response for call::StartTransaction {
    type ResponseType = call_result::StartTransaction;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::StartTransaction(payload),
        ))
    }
}

impl Response for call::StatusNotification {
    type ResponseType = call_result::EmptyResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleEmptyResponse(
                call_result::EmptyResponses::EmptyResponse(payload),
            ),
        ))
    }
}

impl Response for call::StopTransaction {
    type ResponseType = call_result::GenericIdTagInfo;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleEmptyResponse(
                call_result::EmptyResponses::GenericIdTagInfoResponse(payload),
            ),
        ))
    }
}

impl Response for call::TriggerMessage {
    type ResponseType = call_result::GenericStatusResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleStatusResponse(
                call_result::StatusResponses::StatusResponse(payload),
            ),
        ))
    }
}

impl Response for call::UnlockConnector {
    type ResponseType = call_result::GenericStatusResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleStatusResponse(
                call_result::StatusResponses::StatusResponse(payload),
            ),
        ))
    }
}

impl Response for call::UpdateFirmware {
    type ResponseType = call_result::EmptyResponse;
    fn get_response(&self, unique_id: String, payload: Self::ResponseType) -> parse::Message {
        parse::Message::CallResult(call_result::CallResult::new(
            unique_id,
            call_result::ResultPayload::PossibleEmptyResponse(
                call_result::EmptyResponses::EmptyResponse(payload),
            ),
        ))
    }
}
