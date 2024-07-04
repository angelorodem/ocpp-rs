use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString};

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum AuthorizationStatus {
    /// Identifier is allowed for charging.
    #[default]
    Accepted,
    /// Identifier has been blocked. Not allowed for charging.
    Blocked,
    /// Identifier has expired. Not allowed for charging.
    Expired,
    /// Identifier is unknown. Not allowed for charging.
    Invalid,
    /// Identifier is already involved in another transaction and multiple transactions are not allowed. (Only relevant for a StartTransaction.req.)
    ConcurrentTx,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum AvailabilityStatus {
    /// Request has been accepted and will be executed.
    #[default]
    Accepted,
    /// Request has not been accepted and will not be executed.
    Rejected,
    /// Request has been accepted and will be executed when transaction(s) in progress have finished.
    Scheduled,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum AvailabilityType {
    /// Charge point is not available for charging.
    Inoperative,
    /// Charge point is available for charging.
    #[default]
    Operative,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum CancelReservationStatus {
    /// Reservation for the identifier has been cancelled.
    #[default]
    Accepted,
    /// Reservation could not be cancelled, because there is no reservation active for the identifier.
    Rejected,
}
#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum CertificateSignedStatus {
    #[default]
    Accepted,
    Rejected,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum CertificateStatus {
    #[default]
    Accepted,
    Rejected,
    Failed,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum CertificateUse {
    #[default]
    CentralSystemRootCertificate,
    ManufacturerRootCertificate,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default, EnumString)]
pub enum ChargePointErrorCode {
    /// Failure to lock or unlock connector.
    ConnectorLockFailure,
    /// Communication failure with the vehicle, might be Mode 3 or other communication protocol problem. This is not a real error in the sense that the Charge Point doesn’t need to go to the faulted state. Instead, it should go to the SuspendedEVSE state.
    EVCommunicationError,
    /// Ground fault circuit interrupter has been activated.
    GroundFailure,
    /// Temperature inside Charge Point is too high.
    HighTemperature,
    /// Error in internal hard- or software component.
    InternalError,
    /// The authorization information received from the Central System is in conflict with the LocalAuthorizationList.
    LocalListConflict,
    /// No error to report.
    #[default]
    NoError,
    /// Other type of error. More information in vendorErrorCode.
    OtherError,
    /// Over current protection device has tripped.
    OverCurrentFailure,
    /// Voltage has risen above an acceptable level.
    OverVoltage,
    /// Failure to read electrical/energy/power meter.
    PowerMeterFailure,
    /// Failure to control power switch.
    PowerSwitchFailure,
    /// Failure with idTag reader.
    ReaderFailure,
    /// Unable to perform a reset.
    ResetFailure,
    /// Voltage has dropped below an acceptable level.
    UnderVoltage,
    /// Wireless communication device reports a weak signal.
    WeakSignal,
}

#[derive(AsRefStr, EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum ChargePointStatus {
    /// When a Connector becomes available for a new user (Operative)
    #[default]
    Available,
    /// When a Connector becomes no longer available for a new user but there is no ongoing Transaction (yet). Typically a Connector is in preparing state when a user presents a tag, inserts a cable or a vehicle occupies the parking bay 6 (Operative)
    Preparing,
    /// When the contactor of a Connector closes, allowing the vehicle to charge (Operative)
    Charging,
    /// When the EV is connected to the EVSE but the EVSE is not offering energy to the EV, e.g. due to a smart charging restriction, local supply power constraints, or as the result of StartTransaction.conf indicating that charging is not allowed etc. (Operative)
    SuspendedEVSE,
    /// When the EV is connected to the EVSE and the EVSE is offering energy but the EV is not taking any energy. (Operative)
    SuspendedEV,
    /// When a Transaction has stopped at a Connector, but the Connector is not yet available for a new user, e.g. the cable has not been removed or the vehicle has not left the parking bay (Operative)
    Finishing,
    /// When a Connector becomes reserved as a result of a Reserve Now command (Operative)
    Reserved,
    /// When a Connector becomes unavailable as the result of a Change Availability command or an event upon which the Charge Point transitions to unavailable at its discretion. Upon receipt of a Change Availability command, the status MAY change immediately or the change MAY be scheduled. When scheduled, the Status Notification shall be send when the availability change becomes effective (Inoperative).
    Unavailable,
    /// When a Charge Point or connector has reported an error and is not available for energy delivery, (Inoperative)
    Faulted,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum ChargingProfileKindType {
    /// Schedule periods are relative to a fixed point in time defined in the schedule.
    #[default]
    Absolute,
    ///  The schedule restarts periodically at the first schedule period.
    Recurring,
    /// Schedule periods are relative to a situation-specific start point (such as the start of a Transaction) that is determined by the charge point.
    Relative,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum ChargingProfilePurposeType {
    /// Configuration for the maximum power or current available for an entire Charge Point.
    #[default]
    ChargePointMaxProfile,
    /// Default profile *that can be configured in the Charge Point. When a new transaction is started, this profile SHALL be used, unless it was a transaction that was started by a RemoteStartTransaction.req with a ChargeProfile that is accepted by the Charge Point.
    TxDefaultProfile,
    /// Profile with constraints to be imposed by the Charge Point on the current transaction, or on a new transaction when this is started via a RemoteStartTransaction.req with a ChargeProfile. A profile with this purpose SHALL cease to be valid when the transaction terminates.
    TxProfile,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum ChargingProfileStatus {
    /// Request has been accepted and will be executed.
    #[default]
    Accepted,
    /// Request has not been accepted and will not be executed.
    Rejected,
    /// Charge Point indicates that the request is not supported.
    NotSupported,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum ChargingRateUnitType {
    #[default]
    W,
    A,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CiStringType {
    CiString20 = 20,
    CiString25 = 25,
    CiString50 = 50,
    CiString255 = 255,
    CiString500 = 500,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum ClearCacheStatus {
    /// Command has been executed.
    #[default]
    Accepted,
    /// Command has not been executed.
    Rejected,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum ClearChargingProfileStatus {
    /// Request has been accepted and will be executed.
    #[default]
    Accepted,
    /// No Charging Profile(s) were found matching the request.
    Unknown,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum ConfigurationStatus {
    /// Configuration key is supported and setting has been changed.
    #[default]
    Accepted,
    /// Configuration key is supported, but setting could not be changed.
    Rejected,
    /// Configuration key is supported and setting has been changed, but change will be available after reboot (Charge Point will not reboot itself)
    RebootRequired,
    /// Configuration key is not supported.
    NotSupported,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum ConfigurationKey {
    AllowOfflineTxForUnknownId,
    AuthorizationCacheEnabled,
    AuthorizeRemoteTxRequests,
    BlinkRepeat,
    ClockAlignedDataInterval,
    ConnectionTimeOut,
    ConnectorPhaseRotation,
    ConnectorPhaseRotationMaxLength,
    GetConfigurationMaxKeys,
    HeartbeatInterval,
    LightIntensity,
    LocalAuthorizeOffline,
    LocalPreAuthorize,
    MaxEnergyOnInvalidId,
    MeterValuesAlignedData,
    MeterValuesAlignedDataMaxLength,
    MeterValuesSampledData,
    MeterValuesSampledDataMaxLength,
    MeterValueSampleInterval,
    MinimumStatusDuration,
    NumberOfConnectors,
    ResetRetries,
    StopTransactionOnEVSideDisconnect,
    StopTransactionOnInvalidId,
    StopTxnAlignedData,
    StopTxnAlignedDataMaxLength,
    StopTxnSampledData,
    StopTxnSampledDataMaxLength,
    SupportedFeatureProfiles,
    SupportedFeatureProfilesMaxLength,
    TransactionMessageAttempts,
    TransactionMessageRetryInterval,
    UnlockConnectorOnEVSideDisconnect,
    WebSocketPingInterval,
    LocalAuthListEnabled,
    LocalAuthListMaxLength,
    SendLocalListMaxLength,
    ReserveConnectorZeroSupported,
    ChargeProfileMaxStackLevel,
    ChargingScheduleAllowedChargingRateUnit,
    ChargingScheduleMaxPeriods,
    ConnectorSwitch3to1PhaseSupported,
    MaxChargingProfilesInstalled,
    CentralContractValidationAllowed,
    CertificateSignedMaxChainSize,
    CertSigningWaitMinimum,
    CertSigningRepeatTimes,
    CertificateStoreMaxLength,
    ContractValidationOffline,
    ISO15118PnCEnabled,
    AdditionalRootCertificateCheck,
    AuthorizationKey,
    CpoName,
    SecurityProfile,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum DataTransferStatus {
    /// Message has been accepted and the contained request is accepted.
    #[default]
    Accepted,
    /// Message has been accepted but the contained request is rejected.
    Rejected,
    /// Message could not be interpreted due to unknown messageId string.
    UnknownMessageId,
    /// Message could not be interpreted due to unknown vendorId string.
    UnknownVendorId,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum DeleteCertificateStatus {
    #[default]
    Accepted,
    Failed,
    NotFound,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum DiagnosticsStatus {
    /// Charge Point is not performing diagnostics related tasks. Status Idle SHALL only be used as in a DiagnosticsStatusNotification.req that was triggered by a TriggerMessage.req
    #[default]
    Idle,
    /// Diagnostics information has been uploaded.
    Uploaded,
    /// Uploading of diagnostics failed.
    UploadFailed,
    /// File is being uploaded.
    Uploading,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum FirmwareStatus {
    /// New firmware has been downloaded by Charge Point.
    Downloaded,
    /// Charge point failed to download firmware.
    DownloadFailed,
    /// Firmware is being downloaded.
    Downloading,
    /// Charge Point is not performing firmware update related tasks. Status Idle SHALL only be used as in a FirmwareStatusNotificationRequest that was triggered by a TriggerMessageRequest
    #[default]
    Idle,
    /// Installation of new firmware has failed.
    InstallationFailed,
    /// Firmware is being installed.
    Installing,
    /// New firmware has successfully been installed in charge point.
    Installed,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum GenericStatus {
    /// Request has been accepted and will be executed.
    #[default]
    Accepted,
    /// Request has not been accepted and will not be executed.
    Rejected,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum GetCompositeScheduleStatus {
    /// Request has been accepted and will be executed.
    #[default]
    Accepted,
    /// Request has not been accepted and will not be executed.
    Rejected,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum GetInstalledCertificateStatus {
    #[default]
    Accepted,
    NotFound,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum HashAlgorithm {
    SHA256,
    SHA384,
    SHA512,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum Location {
    /// Measurement inside body of Charge Point (e.g. Temperature)
    #[default]
    Body,
    ///Measurement taken from cable between EV and Charge Point
    Cable,
    ///Measurement taken by EV
    #[strum(serialize = "EV")]
    #[serde(rename = "EV")]
    Ev,
    ///Measurement at network (“grid”) inlet connection
    Inlet,
    ///Measurement at a Connector. Default value
    Outlet,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum Log {
    #[default]
    DiagnosticsLog,
    SecurityLog,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum LogStatus {
    #[default]
    Accepted,
    Rejected,
    AcceptedCanceled,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum Measurand {
    ///Instantaneous current flow from EV
    #[strum(serialize = "Current.Export")]
    #[serde(rename = "Current.Export")]
    #[default]
    CurrentExport,
    /// Instantaneous current flow to EV
    #[strum(serialize = "Current.Import")]
    #[serde(rename = "Current.Import")]
    CurrentImport,
    /// Maximum current offered to EV
    #[strum(serialize = "Current.Offered")]
    #[serde(rename = "Current.Offered")]
    CurrentOffered,
    /// Numerical value read from the "active electrical energy" (Wh or kWh) register of the (most authoritative) electrical meter measuring energy exported (to the grid).
    #[strum(serialize = "Energy.Active.Export.Register")]
    #[serde(rename = "Energy.Active.Export.Register")]
    EnergyActiveExportRegister,
    /// Numerical value read from the "active electrical energy" (Wh or kWh) register of the (most authoritative) electrical meter measuring energy imported (from the grid supply).
    #[strum(serialize = "Energy.Active.Import.Register")]
    #[serde(rename = "Energy.Active.Import.Register")]
    EnergyActiveImportRegister,
    ///  Numerical value read from the "reactive electrical energy" (VARh or kVARh) register of the (most authoritative) electrical meter measuring energy exported (to the grid).
    #[strum(serialize = "Energy.Reactive.Export.Register")]
    #[serde(rename = "Energy.Reactive.Export.Register")]
    EnergyReactiveExportRegister,
    /// Numerical value read from the "reactive electrical energy" (VARh or kVARh) register of the (most authoritative) electrical meter measuring energy imported (from the grid supply).
    #[strum(serialize = "Energy.Reactive.Import.Register")]
    #[serde(rename = "Energy.Reactive.Import.Register")]
    EnergyReactiveImportRegister,
    /// Absolute amount of "active electrical energy" (Wh or kWh) exported (to the grid) during an associated time "interval", specified by a Metervalues ReadingContext, and applicable interval duration configuration values (in seconds) for "ClockAlignedDataInterval" and "MeterValueSampleInterval".
    #[strum(serialize = "Energy.Active.Export.Interval")]
    #[serde(rename = "Energy.Active.Export.Interval")]
    EnergyActiveExportInterval,
    /// Absolute amount of "active electrical energy" (Wh or kWh) imported (from the grid supply) during an associated time "interval", specified by a Metervalues ReadingContext, and applicable interval duration configuration values (in seconds) for "ClockAlignedDataInterval" and "MeterValueSampleInterval".
    #[strum(serialize = "Energy.Active.Import.Interval")]
    #[serde(rename = "Energy.Active.Import.Interval")]
    EnergyActiveImportInterval,
    /// Absolute amount of "reactive electrical energy" (VARh or kVARh) exported (to the grid) during an associated time "interval", specified by a Metervalues ReadingContext, and applicable interval duration configuration values (in seconds) for "ClockAlignedDataInterval" and "MeterValueSampleInterval".
    #[strum(serialize = "Energy.Reactive.Export.Interval")]
    #[serde(rename = "Energy.Reactive.Export.Interval")]
    EnergyReactiveExportInterval,
    ///  Absolute amount of "reactive electrical energy" (VARh or kVARh) imported (from the grid supply) during an associated time "interval", specified by a Metervalues ReadingContext, and applicable interval duration configuration values (in seconds) for "ClockAlignedDataInterval" and "MeterValueSampleInterval".
    #[strum(serialize = "Energy.Reactive.Import.Interval")]
    #[serde(rename = "Energy.Reactive.Import.Interval")]
    EnergyReactiveImportInterval,
    /// Instantaneous reading of powerline frequency. NOTE: OCPP 1.6 does not have a UnitOfMeasure for frequency, the UnitOfMeasure for any SampledValue with measurand: Frequency is Hertz.
    Frequency,
    /// Instantaneous active power exported by EV. (W or kW)
    #[strum(serialize = "Power.Active.Export")]
    #[serde(rename = "Power.Active.Export")]
    PowerActiveExport,
    /// Instantaneous active power imported by EV. (W or kW)
    #[strum(serialize = "Power.Active.Import")]
    #[serde(rename = "Power.Active.Import")]
    PowerActiveImport,
    /// Instantaneous power factor of total energy flow
    #[strum(serialize = "Power.Factor")]
    #[serde(rename = "Power.Factor")]
    PowerFactor,
    /// Maximum power offered to EV
    #[strum(serialize = "Power.Offered")]
    #[serde(rename = "Power.Offered")]
    PowerOffered,
    /// Instantaneous reactive power exported by EV. (var or kvar)
    #[strum(serialize = "Power.Reactive.Export")]
    #[serde(rename = "Power.Reactive.Export")]
    PowerReactiveExport,
    /// Instantaneous reactive power imported by EV. (var or kvar)
    #[strum(serialize = "Power.Reactive.Import")]
    #[serde(rename = "Power.Reactive.Import")]
    PowerReactiveImport,
    /// Fan speed in RPM
    #[strum(serialize = "RPM")]
    #[serde(rename = "RPM")]
    Rpm,
    /// State of charge of charging vehicle in percentage
    SoC,
    /// Temperature reading inside Charge Point.
    Temperature,
    /// Instantaneous AC RMS supply voltage
    Voltage,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum MessageTrigger {
    /// To trigger a BootNotification request
    #[default]
    BootNotification,
    ///To trigger a DiagnosticsStatusNotification request
    DiagnosticsStatusNotification,
    /// To trigger a FirmwareStatusNotification request
    FirmwareStatusNotification,
    /// To trigger a Heartbeat request
    Heartbeat,
    /// To trigger a MeterValues request
    MeterValues,
    ///  To trigger a StatusNotification request
    StatusNotification,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum Phase {
    #[default]
    L1,
    L2,
    L3,
    N,
    #[strum(serialize = "L1-N")]
    #[serde(rename = "L1-N")]
    L1N,
    #[strum(serialize = "L2-N")]
    #[serde(rename = "L2-N")]
    L2N,
    #[strum(serialize = "L3-N")]
    #[serde(rename = "L3-N")]
    L3N,
    #[strum(serialize = "L1-L2")]
    #[serde(rename = "L1-L2")]
    L1L2,
    #[strum(serialize = "L2-L3")]
    #[serde(rename = "L2-L3")]
    L2L3,
    #[strum(serialize = "L3-L1")]
    #[serde(rename = "L3-L1")]
    L3L1,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum ReadingContext {
    /// Value taken at start of interruption.
    #[strum(serialize = "Interruption.Begin")]
    #[serde(rename = "Interruption.Begin")]
    InterruptionBegin,
    /// Value taken when resuming after interruption.
    #[strum(serialize = "Interruption.End")]
    #[serde(rename = "Interruption.End")]
    InterruptionEnd,
    /// Value for any other situations.
    #[default]
    Other,
    /// Value taken at clock aligned interval.
    #[strum(serialize = "Sample.Clock")]
    #[serde(rename = "Sample.Clock")]
    SampleClock,
    /// Value taken as periodic sample relative to start time of transaction.
    #[strum(serialize = "Sample.Periodic")]
    #[serde(rename = "Sample.Periodic")]
    SamplePeriodic,
    /// Value taken at start of transaction.
    #[strum(serialize = "Transaction.Begin")]
    #[serde(rename = "Transaction.Begin")]
    TransactionBegin,
    /// Value taken at end of transaction.
    #[strum(serialize = "Transaction.End")]
    #[serde(rename = "Transaction.End")]
    TransactionEnd,
    /// Value taken in response to a TriggerMessage.req
    Trigger,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default, Copy)]
pub enum Reason {
    /// The transaction was stopped because of the authorization status in a StartTransaction.conf
    DeAuthorized,
    /// Emergency stop button was used.
    EmergencyStop,
    /// disconnecting of cable, vehicle moved away from inductive charge unit.
    EVDisconnected,
    /// A hard reset command was received.
    HardReset,
    /// Stopped locally on request of the user at the Charge Point. This is a regular termination of a transaction. Examples: presenting an RFID tag, pressing a button to stop.
    Local,
    /// Any other reason.
    #[default]
    Other,
    /// Complete loss of power.
    PowerLoss,
    /// A locally initiated reset/reboot occurred. (for instance watchdog kicked in)
    Reboot,
    /// Stopped remotely on request of the user. This is a regular termination of a transaction. Examples: termination using a smartphone app, exceeding a (non local) prepaid credit.
    Remote,
    /// A soft reset command was received.
    SoftReset,
    /// Central System sent an Unlock Connector command.
    UnlockCommand,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum RecurrencyKind {
    ///  The schedule restarts every 24 hours, at the same time as in the startSchedule.
    #[default]
    Daily,
    /// The schedule restarts every 7 days, at the same time and day-of-the-week as in the startSchedule.
    Weekly,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum RegistrationStatus {
    /// Charge point is accepted by Central System.
    #[default]
    Accepted,
    /// Central System is not yet ready to accept the Charge Point. Central System may send
    /// messages to retrieve information or prepare the Charge Point.
    Pending,
    /// Charge point is not accepted by Central System. This may happen when the Charge Point id is
    /// not known by Central System.
    Rejected,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum RemoteStartStopStatus {
    /// Command will be executed.
    #[default]
    Accepted,
    /// Command will not be executed.
    Rejected,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum ReservationStatus {
    /// Reservation has been made.
    #[default]
    Accepted,
    /// Reservation has not been made, because connectors or specified connector are in a faulted state.
    Faulted,
    /// Reservation has not been made. All connectors or the specified connector are occupied.
    Occupied,
    /// Reservation has not been made. Charge Point is not configured to accept reservations.
    Rejected,
    /// Reservation has not been made, because connectors or specified connector are in an unavailable state.
    Unavailable,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum ResetStatus {
    /// Command will be executed.
    #[default]
    Accepted,
    /// Command will not be executed.
    Rejected,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum ResetType {
    /// Restart (all) the hardware, the Charge Point is not required to gracefully stop ongoing transaction. If possible the Charge Point sends a StopTransaction.req for previously ongoing transactions after having restarted and having been accepted by the Central System via a BootNotification.conf. This is a last resort solution for a not correctly functioning Charge Point, by sending a "hard" reset, (queued) information might get lost.
    Hard,
    /// Stop ongoing transactions gracefully and sending StopTransaction.req for every ongoing transaction. It should then restart the application software (if possible, otherwise restart the processor/controller).
    #[default]
    Soft,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum TriggerMessageStatus {
    /// Requested notification will be sent.
    #[default]
    Accepted,
    /// Requested notification will not be sent.
    Rejected,
    /// Requested notification cannot be sent because it is either not implemented or unknown.
    NotImplemented,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum UnitOfMeasure {
    /// Watt-hours (energy). Default.
    Wh,
    /// kiloWatt-hours (energy).
    #[default]
    #[serde(rename = "kWh")]
    #[strum(serialize = "kWh")]
    KWh,
    /// Var-hours (reactive energy).
    #[serde(rename = "varh")]
    #[strum(serialize = "varh")]
    Varh,
    /// kilovar-hours (reactive energy).
    #[serde(rename = "kvarh")]
    #[strum(serialize = "kvarh")]
    Kvarh,
    /// Watts (power).
    W,
    /// kilowatts (power).
    #[serde(rename = "kW")]
    #[strum(serialize = "kW")]
    Kw,
    /// VoltAmpere (apparent power).
    #[serde(rename = "VA")]
    #[strum(serialize = "VA")]
    Va,
    /// kiloVolt Ampere (apparent power).
    #[serde(rename = "kVA")]
    #[strum(serialize = "kVA")]
    Kva,
    /// Vars (reactive power).
    #[serde(rename = "var")]
    #[strum(serialize = "var")]
    Var,
    /// kilovars (reactive power).
    #[serde(rename = "kvar")]
    #[strum(serialize = "kvar")]
    Kvar,
    /// Amperes (current).
    A,
    /// Voltage (r.m.s. AC).
    V,
    /// Degrees (temperature).
    Celsius,
    /// Degrees (temperature).
    Fahrenheit,
    /// Degrees Kelvin (temperature).
    K,
    /// Percentage.
    Percent,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum UnlockStatus {
    /// # From OCPP Specification
    /// Connector has successfully been unlocked.
    #[default]
    Unlocked,
    /// # From OCPP Specification
    /// Failed to unlock the connector: The Charge Point has tried to unlock the connector and has
    /// detected that the connector is still locked or the unlock mechanism failed.
    UnlockFailed,
    /// # From OCPP Specification
    /// Charge Point has no connector lock, or ConnectorId is unknown.
    NotSupported,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum UpdateFirmwareStatus {
    #[default]
    Accepted,
    Rejected,
    AcceptedCanceled,
    InvalidCertificate,
    RevokedCertificate,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum UploadLogStatus {
    BadMessage,
    #[default]
    Idle,
    NotSupportedOperation,
    PermissionDenied,
    Uploaded,
    UploadFailure,
    Uploading,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum UpdateStatus {
    /// Local Authorization List successfully updated.
    #[default]
    Accepted,
    /// Failed to update the Local Authorization List.
    Failed,
    /// Update of Local Authorization List is not supported by Charge Point.
    NotSupported,
    /// Version number in the request for a differential update is less or equal then version number of current list.
    VersionMismatch,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum UpdateType {
    /// Indicates that the current Local Authorization List must be updated with the values in this message.
    Differential,
    /// Indicates that the current Local Authorization List must be replaced by the values in this message.
    #[default]
    Full,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum ValueFormat {
    /// Data is to be interpreted as integer/decimal numeric data.
    #[default]
    Raw,
    /// Data is represented as a signed binary data block, encoded as hex data.
    SignedData,
}
