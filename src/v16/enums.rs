use strum_macros::EnumString;
use serde::{Deserialize, Serialize};

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum Action {
    Authorize,
    BootNotification,
    CancelReservation,
    CertificateSigned,
    ChangeAvailability,
    ChangeConfiguration,
    ClearCache,
    ClearChargingProfile,
    DataTransfer,
    DeleteCertificate,
    DiagnosticsStatusNotification,
    ExtendedTriggerMessage,
    FirmwareStatusNotification,
    GetCompositeSchedule,
    GetConfiguration,
    GetDiagnostics,
    GetInstalledCertificateIds,
    GetLocalListVersion,
    GetLog,
    Heartbeat,
    InstallCertificate,
    LogStatusNotification,
    MeterValues,
    RemoteStartTransaction,
    RemoteStopTransaction,
    ReserveNow,
    Reset,
    SecurityEventNotification,
    SendLocalList,
    SetChargingProfile,
    SignCertificate,
    SignedFirmwareStatusNotification,
    SignedUpdateFirmware,
    StartTransaction,
    StatusNotification,
    StopTransaction,
    TriggerMessage,
    UnlockConnector,
    UpdateFirmware,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum AuthorizationStatus {
    Accepted,
    Blocked,
    Expired,
    Invalid,
    ConcurrentTx,
}


#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum AvailabilityStatus {
        Accepted,
        Rejected,
        Scheduled,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum AvailabilityType {
        Inoperative,
        Operative,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum CancelReservationStatus {
        Accepted,
        Rejected,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum CertificateSignedStatus {
        Accepted,
        Rejected,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum CertificateStatus {
        Accepted,
        Rejected,
        Failed,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum CertificateUse {
    CentralSystemRootCertificate,
    ManufacturerRootCertificate,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum ChargePointErrorCode {
    ConnectorLockFailure,
    EVCommunicationError,
    GroundFailure,
    HighTemperature,
    InternalError,
    LocalListConflict,
    NoError,
    OtherError,
    OverCurrentFailure,
    OverVoltage,
    PowerMeterFailure,
    PowerSwitchFailure,
    ReaderFailure,
    ResetFailure,
    UnderVoltage,
    WeakSignal,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum ChargePointStatus {
    Available,
    Preparing,
    Charging,
    SuspendedEVSE,
    SuspendedEV,
    Finishing,
    Reserved,
    Unavailable,
    Faulted,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum ChargingProfileKindType {
    Absolute,
    Recurring,
    Relative,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum ChargingProfilePurposeType {
    ChargePointMaxProfile,
    TxDefaultProfile,
    TxProfile,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum ChargingProfileStatus {
    Accepted,
    Rejected,
    NotSupported,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum ChargingRateUnitType {
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

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum ClearCacheStatus {
    Accepted,
    Rejected,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum ClearChargingProfileStatus {
    Accepted,
    Unknown,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum ConfigurationStatus {
    Accepted,
    Rejected,
    RebootRequired,
    NotSupported,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
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

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum DataTransferStatus {
    Accepted,
    Rejected,
    UnknownMessageId,
    UnknownVendorId,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum DeleteCertificateStatus {
    Accepted,
    Failed,
    NotFound,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum DiagnosticsStatus {
    Idle,
    Uploaded,
    UploadFailed,
    Uploading,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum FirmwareStatus {
    Downloaded,
    DownloadFailed,
    Downloading,
    Idle,
    InstallationFailed,
    Installing,
    Installed,
    DownloadScheduled,
    DownloadPaused,
    InstallRebooting,
    InstallScheduled,
    InstallVerificationFailed,
    InvalidSignature,
    SignatureVerified,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum GenericStatus {
    Accepted,
    Rejected,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum GetCompositeScheduleStatus {
    Accepted,
    Rejected,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum GetInstalledCertificateStatus {
    Accepted,
    NotFound,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum HashAlgorithm {
    SHA256,
    SHA384,
    SHA512,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum Location {
    Inlet,
    Outlet,
    Body,
    Cable,
    EV,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum Log {
    DiagnosticsLog,
    SecurityLog,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum LogStatus {
    Accepted,
    Rejected,
    AcceptedCanceled,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum Measurand {
    CurrentExport,
    CurrentImport,
    CurrentOffered,
    EnergyActiveExportRegister,
    EnergyActiveImportRegister,
    EnergyReactiveExportRegister,
    EnergyReactiveImportRegister,
    EnergyActiveExportInterval,
    EnergyActiveImportInterval,
    EnergyReactiveExportInterval,
    EnergyReactiveImportInterval,
    Frequency,
    PowerActiveExport,
    PowerActiveImport,
    PowerFactor,
    PowerOffered,
    PowerReactiveExport,
    PowerReactiveImport,
    RPM,
    SoC,
    Temperature,
    Voltage,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum MessageTrigger {
    BootNotification,
    FirmwareStatusNotification,
    Heartbeat,
    MeterValues,
    StatusNotification,
    DiagnosticsStatusNotification,
    LogStatusNotification,
    SignChargePointCertificate,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum Phase {
    L1,
    L2,
    L3,
    N,
    L1N,
    L2N,
    L3N,
    L1L2,
    L2L3,
    L3L1,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum ReadingContext {
    InterruptionBegin,
    InterruptionEnd,
    Other,
    SampleClock,
    SamplePeriodic,
    TransactionBegin,
    TransactionEnd,
    Trigger,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum Reason {
    EmergencyStop,
    EVDisconnected,
    HardReset,
    Local,
    Other,
    PowerLoss,
    Reboot,
    Remote,
    SoftReset,
    UnlockCommand,
    DeAuthorized,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum RecurrencyKind {
    Daily,
    Weekly,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum RegistrationStatus {
    Accepted,
    Pending,
    Rejected,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum RemoteStartStopStatus {
    Accepted,
    Rejected,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum ReservationStatus {
    Accepted,
    Faulted,
    Occupied,
    Rejected,
    Unavailable,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum ResetStatus {
    Accepted,
    Rejected,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum ResetType {
    Hard,
    Soft,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum TriggerMessageStatus {
    Accepted,
    Rejected,
    NotImplemented,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum UnitOfMeasure {
    Wh,
    KWh,
    Varh,
    KVarh,
    W,
    KW,
    VA,
    KVA,
    Var,
    KVar,
    A,
    V,
    Celsius,
    Fahrenheit,
    K,
    Percent,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum UnlockStatus {
    Unlocked,
    UnlockFailed,
    NotSupported,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum UpdateFirmwareStatus {
    Accepted,
    Rejected,
    AcceptedCanceled,
    InvalidCertificate,
    RevokedCertificate,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum UploadLogStatus {
    BadMessage,
    Idle,
    NotSupportedOperation,
    PermissionDenied,
    Uploaded,
    UploadFailure,
    Uploading,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum UpdateStatus {
    Accepted,
    Failed,
    NotSupported,
    VersionMismatch,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum UpdateType {
    Differential,
    Full,
}

#[derive(EnumString, Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum ValueFormat {
    Raw,
    SignedData,
}