#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventCode {
    StartOfMessages,
    StartOfSystemHours,
    StartOfMarketHours,
    EndOfMarketHours,
    EndOfSystemHours,
    EndOfMessages,
    EmergencyHalt,
    EmergencyQuoteOnly,
    EmergencyResumption,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarketCategory {
    Shfe,
    Dce,
    Czce,
    Cffex,
    Gce,
    Sse,
    Szse,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueClassification {
    AmericanDepositaryShare,
    Bond,
    CommonStock,
    Futures,
    Options,
    DepositoryReceipt,
    OrdinaryShare,
    PreferredStock,
    OtherSecurities,
    Right,
    ConvertibleDebenture,
    Unit,
    UnitsPerBenifInt,
    Warrant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarketMakerMode {
    Normal,
    Passive,
    Syndicate,
    Presyndicate,
    Penalty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarketParticipantState {
    Active,
    Excused,
    Withdrawn,
    Suspended,
    Deleted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradingState {
    Halted,
    Paused,
    QuotationOnly,
    Trading,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
    BuyCover,
    SellClose,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImbalanceDirection {
    Buy,
    Sell,
    NoImbalance,
    InsufficientOrders,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossType {
    Opening,
    Closing,
    Halted,
    Intraday,
}
