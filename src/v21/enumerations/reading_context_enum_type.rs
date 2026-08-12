//! `ReadingContextEnumType`

crate::lenient_str_enum! {
    pub enum ReadingContextEnumType {
        InterruptionBegin => "Interruption.Begin",
        InterruptionEnd => "Interruption.End",
        Other,
        SampleClock => "Sample.Clock",
        SamplePeriodic => "Sample.Periodic",
        TransactionBegin => "Transaction.Begin",
        TransactionEnd => "Transaction.End",
        Trigger,
    }
    @unknown Unknown
}
