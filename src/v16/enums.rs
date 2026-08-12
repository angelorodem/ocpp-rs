use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum AvailabilityType {
    /// Charge point is not available for charging.    
    Inoperative,
    /// Charge point is available for charging.    
    #[default]
    Operative,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum CertificateUse {
    #[default]
    CentralSystemRootCertificate,
    ManufacturerRootCertificate,
}

crate::lenient_str_enum! {
    /// Charge Point error reported in `StatusNotification.errorCode`.
    ///
    /// Unknown / vendor-specific wire strings are accepted as [`ChargePointErrorCode::Unknown`].
    @default
    pub enum ChargePointErrorCode {
        /// Failure to lock or unlock connector.
        ConnectorLockFailure,
        /// Communication failure with the vehicle, might be Mode 3 or other communication protocol problem.\\
        /// This is not a real error in the sense that the Charge Point doesn’t need to go to the faulted state. Instead, it should go to the `SuspendedEVSE` state.
        EVCommunicationError,
        /// Ground fault circuit interrupter has been activated.
        GroundFailure,
        /// Temperature inside Charge Point is too high.
        HighTemperature,
        /// Error in internal hard- or software component.
        InternalError,
        /// The authorization information received from the Central System is in conflict with the `LocalAuthorizationList`.
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
    @unknown Unknown
}

crate::lenient_str_enum! {
    /// Charge Point connector status in `StatusNotification.status`.
    ///
    /// Unknown / vendor-specific wire strings are accepted as [`ChargePointStatus::Unknown`].
    @default
    pub enum ChargePointStatus {
        /// When a Connector becomes available for a new user (Operative)
        #[default]
        Available,
        /// When a Connector becomes no longer available for a new user but there is no ongoing Transaction (yet).\\
        ///  Typically a Connector is in preparing state when a user presents a tag, inserts a cable or a vehicle occupies the parking bay 6 (Operative)
        Preparing,
        /// When the contactor of a Connector closes, allowing the vehicle to charge (Operative)
        Charging,
        /// When the EV is connected to the EVSE but the EVSE is not offering energy to the EV, e.g. due to a smart charging restriction,\\
        ///  local supply power constraints, or as the result of StartTransaction.conf indicating that charging is not allowed etc. (Operative)
        SuspendedEVSE,
        /// When the EV is connected to the EVSE and the EVSE is offering energy but the EV is not taking any energy. (Operative)
        SuspendedEV,
        /// When a Transaction has stopped at a Connector, but the Connector is not yet available for a new user, e.g. the cable has not been removed or the vehicle has not left the parking bay (Operative)
        Finishing,
        /// When a Connector becomes reserved as a result of a Reserve Now command (Operative)
        Reserved,
        /// When a Connector becomes unavailable as the result of a Change Availability command or an event upon which the Charge Point transitions to unavailable at its discretion.\\
        /// Upon receipt of a Change Availability command, the status MAY change immediately or the change MAY be scheduled.\\
        ///  When scheduled, the Status Notification shall be send when the availability change becomes effective (Inoperative).
        Unavailable,
        /// When a Charge Point or connector has reported an error and is not available for energy delivery, (Inoperative)
        Faulted,
    }
    @unknown Unknown
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
    /// Default profile *that can be configured in the Charge Point.\\
    /// When a new transaction is started, this profile SHALL be used, unless it was a transaction that was started by a RemoteStartTransaction.req with a `ChargeProfile` that is accepted by the Charge Point.    
    TxDefaultProfile,
    /// Profile with constraints to be imposed by the Charge Point on the current transaction, or on a new transaction when this is started via a RemoteStartTransaction.req with a `ChargeProfile`.\\
    ///  A profile with this purpose SHALL cease to be valid when the transaction terminates.    
    TxProfile,
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

crate::lenient_str_enum! {
    /// Diagnostics upload status.
    @default
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
    @unknown Unknown
}

crate::lenient_str_enum! {
    /// Firmware update status (core + security extension).
    @default
    pub enum FirmwareStatus {
        /// New firmware has been downloaded by Charge Point.
        Downloaded,
        /// Charge point failed to download firmware.
        DownloadFailed,
        /// Firmware is being downloaded.
        Downloading,
        /// Charge Point is not performing firmware update related tasks. Status Idle SHALL only be used as in a `FirmwareStatusNotificationRequest` that was triggered by a `TriggerMessageRequest`
        #[default]
        Idle,
        /// Installation of new firmware has failed.
        InstallationFailed,
        /// Firmware is being installed.
        Installing,
        /// New firmware has successfully been installed in charge point.
        Installed,
        /// Download of the signed firmware has been scheduled.
        DownloadScheduled,
        /// Download of the signed firmware has been paused.
        DownloadPaused,
        /// Charge Point is about to reboot to activate firmware.
        InstallRebooting,
        /// Installation of the signed firmware has been scheduled.
        InstallScheduled,
        /// Verification of the installed firmware failed.
        InstallVerificationFailed,
        /// Signature of firmware failed verification.
        InvalidSignature,
        /// Signature of firmware successfully verified.
        SignatureVerified,
    }
    @unknown Unknown
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Default)]
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

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Default)]
pub enum RegistrationStatus {
    /// Charge Point is accepted by Central System.    
    #[default]
    Accepted,
    /// Central System is not yet ready to accept the Charge Point. Central System may send\\
    /// messages to retrieve information or prepare the Charge Point.    
    Pending,
    /// Charge Point is not accepted by Central System.    
    Rejected,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Default)]
pub enum AvailabilityStatus {
    /// Request has been accepted and will be executed.    
    #[default]
    Accepted,
    /// Request has not been accepted and will not be executed.    
    Rejected,
    /// Request has been accepted and will be executed when transaction(s) in progress have finished.    
    Scheduled,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Default)]
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

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Default)]
pub enum ClearCacheStatus {
    /// Request has been accepted and will be executed.    
    #[default]
    Accepted,
    /// Request has not been accepted and will not be executed.    
    Rejected,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Default)]
pub enum ClearChargingProfileStatus {
    /// Request has been accepted and will be executed.    
    #[default]
    Accepted,
    /// No Charging Profile(s) were found matching the request.    
    Unknown,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Default)]
pub enum ChargingProfileStatus {
    /// Request has been accepted and will be executed.    
    #[default]
    Accepted,
    /// Request has not been accepted and will not be executed.    
    Rejected,
    /// Charge Point indicates that the request is not supported.    
    NotSupported,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Default)]
pub enum DataTransferStatus {
    /// Message has been accepted and the contained request is accepted.    
    #[default]
    Accepted,
    /// Message has been accepted and the contained request is rejected.    
    Rejected,
    /// Message could not be interpreted due to unknown messageId string.    
    UnknownMessageId,
    /// Message could not be interpreted due to unknown vendorId string.    
    UnknownVendorId,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Default)]
pub enum CancelReservationStatus {
    /// Reservation for the identifier has been cancelled.    
    #[default]
    Accepted,
    /// Reservation could not be cancelled, because there is no reservation active for the identifier.    
    Rejected,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Default)]
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

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Default)]
pub enum RemoteStartStopStatus {
    /// Command will be executed.    
    #[default]
    Accepted,
    /// Command will not be executed.    
    Rejected,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Default)]
pub enum ResetStatus {
    /// Command will be executed.    
    #[default]
    Accepted,
    /// Command will not be executed.    
    Rejected,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Default)]
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

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Default)]
pub enum TriggerMessageStatus {
    /// Requested notification will be sent.    
    #[default]
    Accepted,
    /// Requested notification will not be sent.    
    Rejected,
    /// Requested notification cannot be sent because it is either not implemented or unknown.    
    NotImplemented,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Default)]
pub enum UnlockStatus {
    /// Connector has successfully been unlocked.    
    #[default]
    Unlocked,
    /// Failed to unlock the connector: The Charge Point has tried to unlock the connector and has\\
    /// detected that the connector is still locked or the unlock mechanism failed.    
    UnlockFailed,
    /// Charge Point has no connector lock.    
    NotSupported,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Default)]
pub enum GetCompositeScheduleStatus {
    /// Request has been accepted and will be executed.    
    #[default]
    Accepted,
    /// Request has not been accepted and will not be executed.    
    Rejected,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Default)]
pub enum CertificateSignedStatus {
    /// Signed certificate is valid.    
    #[default]
    Accepted,
    /// Signed certificate is invalid.    
    Rejected,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Default)]
pub enum DeleteCertificateStatus {
    /// Normal successful completion (no errors).    
    #[default]
    Accepted,
    /// Processing failure.    
    Failed,
    /// Requested resource not found.    
    NotFound,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Default)]
pub enum GenericCertificateStatus {
    /// Request has been accepted and will be executed.    
    #[default]
    Accepted,
    /// Processing failure.    
    Failed,
    /// Request has not been accepted and will not be executed.    
    Rejected,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Default)]
pub enum LogStatus {
    /// Accepted this log upload. This does not mean the log file is uploaded is complete, and the\\
    /// Charge Point can still request another log upload.    
    #[default]
    Accepted,
    /// Request has not been accepted and will not be executed.    
    Rejected,
    /// Accepted this log upload, but in doing this has cancelled an ongoing log file upload.    
    AcceptedCanceled,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Default)]
pub enum UpdateFirmwareStatus {
    /// Accepted this firmware update request. This does not mean the firmware update is complete,\\
    /// and the Charge Point can still request another firmware update.    
    #[default]
    Accepted,
    /// Firmware update request has not been accepted and will not be executed.    
    Rejected,
    /// Accepted this firmware update request, but in doing this has cancelled an ongoing firmware update.    
    AcceptedCanceled,
    /// The certificate is invalid.    
    InvalidCertificate,
    /// Failure end state. The Firmware signature is valid.    
    RevokedCertificate,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum GenericAcceptedRejected {
    /// Request has been accepted and will be executed.    
    #[default]
    Accepted,
    /// Request has not been accepted and will not be executed.    
    Rejected,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Default)]
pub enum GetInstalledCertificateStatus {
    /// Normal successful completion (no errors).    
    #[default]
    Accepted,
    /// Requested resource not found.    
    NotFound,
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum HashAlgorithm {
    SHA256,
    SHA384,
    SHA512,
}

crate::lenient_str_enum! {
    /// Measurement location for a `SampledValue`.
    @default
    pub enum Location {
        /// Measurement inside body of Charge Point (e.g. Temperature)
        Body,
        ///Measurement taken from cable between EV and Charge Point
        Cable,
        ///Measurement taken by EV
        Ev => "EV",
        ///Measurement at network (“grid”) inlet connection
        Inlet,
        ///Measurement at a Connector. Default value
        #[default]
        Outlet,
    }
    @unknown Unknown
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum Log {
    #[default]
    DiagnosticsLog,
    SecurityLog,
}

crate::lenient_str_enum! {
    /// Type of measurement in a `SampledValue`.
    @default
    pub enum Measurand {
        ///Instantaneous current flow from EV
        CurrentExport => "Current.Export",
        /// Instantaneous current flow to EV
        CurrentImport => "Current.Import",
        /// Maximum current offered to EV
        CurrentOffered => "Current.Offered",
        /// Numerical value read from the "active electrical energy" (Wh or kWh) register of the (most authoritative) electrical meter measuring energy exported (to the grid).
        EnergyActiveExportRegister => "Energy.Active.Export.Register",
        /// Numerical value read from the "active electrical energy" (Wh or kWh) register of the (most authoritative) electrical meter measuring energy imported (from the grid supply).
        #[default]
        EnergyActiveImportRegister => "Energy.Active.Import.Register",
        ///  Numerical value read from the "reactive electrical energy" (`VARh` or kVARh) register of the (most authoritative) electrical meter measuring energy exported (to the grid).
        EnergyReactiveExportRegister => "Energy.Reactive.Export.Register",
        /// Numerical value read from the "reactive electrical energy" (`VARh` or kVARh) register of the (most authoritative) electrical meter measuring energy imported (from the grid supply).
        EnergyReactiveImportRegister => "Energy.Reactive.Import.Register",
        /// Absolute amount of "active electrical energy" (Wh or kWh) exported (to the grid) during an associated time "interval",\\
        ///  specified by a Metervalues `ReadingContext`, and applicable interval duration configuration values (in seconds) for "`ClockAlignedDataInterval`" and "`MeterValueSampleInterval`".
        EnergyActiveExportInterval => "Energy.Active.Export.Interval",
        /// Absolute amount of "active electrical energy" (Wh or kWh) imported (from the grid supply) during an associated time "interval",\\
        ///  specified by a Metervalues `ReadingContext`, and applicable interval duration configuration values (in seconds) for "`ClockAlignedDataInterval`" and "`MeterValueSampleInterval`".
        EnergyActiveImportInterval => "Energy.Active.Import.Interval",
        /// Absolute amount of "reactive electrical energy" (`VARh` or kVARh) exported (to the grid) during an associated time "interval",\\
        ///  specified by a Metervalues `ReadingContext`, and applicable interval duration configuration values (in seconds) for "`ClockAlignedDataInterval`" and "`MeterValueSampleInterval`".
        EnergyReactiveExportInterval => "Energy.Reactive.Export.Interval",
        ///  Absolute amount of "reactive electrical energy" (`VARh` or kVARh) imported (from the grid supply) during an associated time "interval",\\
        ///  specified by a Metervalues `ReadingContext`, and applicable interval duration configuration values (in seconds) for "`ClockAlignedDataInterval`" and "`MeterValueSampleInterval`".
        EnergyReactiveImportInterval => "Energy.Reactive.Import.Interval",
        /// Instantaneous reading of powerline frequency. NOTE: OCPP 1.6 does not have a `UnitOfMeasure` for frequency,\\
        ///  the `UnitOfMeasure` for any `SampledValue` with measurand: Frequency is Hertz.
        Frequency,
        /// Instantaneous active power exported by EV. (W or kW)
        PowerActiveExport => "Power.Active.Export",
        /// Instantaneous active power imported by EV. (W or kW)
        PowerActiveImport => "Power.Active.Import",
        /// Instantaneous power factor of total energy flow
        PowerFactor => "Power.Factor",
        /// Maximum power offered to EV
        PowerOffered => "Power.Offered",
        /// Instantaneous reactive power exported by EV. (var or kvar)
        PowerReactiveExport => "Power.Reactive.Export",
        /// Instantaneous reactive power imported by EV. (var or kvar)
        PowerReactiveImport => "Power.Reactive.Import",
        /// Fan speed in RPM
        Rpm => "RPM",
        /// State of charge of charging vehicle in percentage
        SoC,
        /// Temperature reading inside Charge Point.
        Temperature,
        /// Instantaneous AC RMS supply voltage
        Voltage,
    }
    @unknown Unknown
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum MessageTrigger {
    /// To trigger a `BootNotification` request    
    #[default]
    BootNotification,
    ///To trigger a `DiagnosticsStatusNotification` request    
    DiagnosticsStatusNotification,
    /// To trigger a `FirmwareStatusNotification` request    
    FirmwareStatusNotification,
    /// To trigger a Heartbeat request    
    Heartbeat,
    /// To trigger a `MeterValues` request    
    MeterValues,
    ///  To trigger a `StatusNotification` request    
    StatusNotification,
    // --- OCPP 1.6 security extension (`ExtendedTriggerMessage`) ---
    /// To trigger a `LogStatusNotification` request    
    LogStatusNotification,
    /// To trigger a `SignCertificate` request (charge point CSR)    
    SignChargePointCertificate,
}

crate::lenient_str_enum! {
    /// Electrical phase for a `SampledValue`.
    @default
    pub enum Phase {
        #[default]
        L1,
        L2,
        L3,
        N,
        L1N => "L1-N",
        L2N => "L2-N",
        L3N => "L3-N",
        L1L2 => "L1-L2",
        L2L3 => "L2-L3",
        L3L1 => "L3-L1",
    }
    @unknown Unknown
}

crate::lenient_str_enum! {
    /// Reading context for a `SampledValue`.
    @default
    pub enum ReadingContext {
        /// Value taken at start of interruption.
        InterruptionBegin => "Interruption.Begin",
        /// Value taken when resuming after interruption.
        InterruptionEnd => "Interruption.End",
        /// Value for any other situations.
        Other,
        /// Value taken at clock aligned interval.
        SampleClock => "Sample.Clock",
        /// Value taken as periodic sample relative to start time of transaction.
        #[default]
        SamplePeriodic => "Sample.Periodic",
        /// Value taken at start of transaction.
        TransactionBegin => "Transaction.Begin",
        /// Value taken at end of transaction.
        TransactionEnd => "Transaction.End",
        /// Value taken in response to a TriggerMessage.req
        Trigger,
    }
    @unknown Unknown
}

crate::lenient_str_enum! {
    /// Reason for stopping a transaction (`StopTransaction.reason`).
    ///
    /// Unknown / vendor-specific wire strings are accepted as [`Reason::Unknown`] so a
    /// proprietary `reason` cannot fail the whole `StopTransaction` parse.
    @default
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
    @unknown Unknown
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
pub enum ResetType {
    /// Restart (all) the hardware, the Charge Point is not required to gracefully stop ongoing transaction.\\
    ///  If possible the Charge Point sends a StopTransaction.req for previously ongoing transactions after having restarted and having been accepted by the Central System via a BootNotification.conf.\\
    ///  This is a last resort solution for a not correctly functioning Charge Point, by sending a "hard" reset, (queued) information might get lost.    
    Hard,
    /// Stop ongoing transactions gracefully and sending StopTransaction.req for every ongoing transaction. It should then restart the application software (if possible, otherwise restart the processor/controller).    
    #[default]
    Soft,
}

crate::lenient_str_enum! {
    /// Unit of measure for a `SampledValue`.
    @default
    pub enum UnitOfMeasure {
        /// Watt-hours (energy). Default.
        #[default]
        Wh,
        /// kiloWatt-hours (energy).
        KWh => "kWh",
        /// Var-hours (reactive energy).
        Varh => "varh",
        /// kilovar-hours (reactive energy).
        Kvarh => "kvarh",
        /// Watts (power).
        W,
        /// kilowatts (power).
        Kw => "kW",
        /// `VoltAmpere` (apparent power).
        Va => "VA",
        /// kiloVolt Ampere (apparent power).
        Kva => "kVA",
        /// Vars (reactive power).
        Var => "var",
        /// kilovars (reactive power).
        Kvar => "kvar",
        /// Amperes (current).
        A,
        /// Voltage (r.m.s. AC).
        V,
        /// Degrees (temperature).
        Celsius => "Celsius" | "Celcius",
        /// Degrees (temperature).
        Fahrenheit,
        /// Degrees Kelvin (temperature).
        K,
        /// Percentage.
        Percent,
    }
    @unknown Unknown
}

crate::lenient_str_enum! {
    /// Log upload status.
    @default
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
    @unknown Unknown
}

#[derive(AsRefStr, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub enum UpdateType {
    /// Indicates that the current Local Authorization List must be updated with the values in this message.    
    Differential,
    /// Indicates that the current Local Authorization List must be replaced by the values in this message.    
    #[default]
    Full,
}

crate::lenient_str_enum! {
    /// Format of a `SampledValue` reading.
    @default
    pub enum ValueFormat {
        /// Data is to be interpreted as integer/decimal numeric data.
        #[default]
        Raw,
        /// Data is represented as a signed binary data block, encoded as hex data.
        SignedData,
    }
    @unknown Unknown
}
