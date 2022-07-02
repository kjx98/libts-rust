use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
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

impl fmt::Display for EventCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventCode::StartOfMessages => write!(f, "Start Of Messages"),
            EventCode::StartOfSystemHours => write!(f, "Start Of SystemHours"),
            EventCode::StartOfMarketHours => write!(f, "Start Of MarketHours"),
            EventCode::EndOfMarketHours => write!(f, "End Of MarketHours"),
            EventCode::EndOfSystemHours => write!(f, "End Of SystemHours"),
            EventCode::EndOfMessages => write!(f, "End Of Messages"),
            EventCode::EmergencyHalt => write!(f, "Emergency Halt"),
            EventCode::EmergencyQuoteOnly => write!(f, "Emergency QuoteOnly"),
            EventCode::EmergencyResumption => write!(f, "Emergency Resumption"),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
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

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
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

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MarketMakerMode {
    Normal,
    Passive,
    Syndicate,
    Presyndicate,
    Penalty,
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MarketParticipantState {
    Active,
    Excused,
    Withdrawn,
    Suspended,
    Deleted,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TradingState {
    Halted,
    PreAuction,
    Auction,
    Paused,
    Trading,
    Break,
}

impl fmt::Display for TradingState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TradingState::Halted => write!(f, "Halted"),
            TradingState::PreAuction => write!(f, "PreAuction"),
            TradingState::Auction => write!(f, "Auction"),
            TradingState::Paused => write!(f, "Paused"),
            TradingState::Trading => write!(f, "Trading"),
            TradingState::Break => write!(f, "Break"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
    BuyCover,
    SellClose,
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Side::Buy => write!(f, "Buy"),
            Side::Sell => write!(f, "Sell"),
            Side::BuyCover => write!(f, "BuyCover"),
            Side::SellClose => write!(f, "SellClose"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CancelReason {
    ByUser,
    Arb,
    ByModifyOrder,
    OddLot,
    OutOfPriceBand,
    BrokenSession,
    OutOfNormalTrading,
}

impl fmt::Display for CancelReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CancelReason::ByUser => write!(f, "ByUser"),
            CancelReason::Arb => write!(f, "By ARB"),
            CancelReason::ByModifyOrder => write!(f, "By ModifyOrder"),
            CancelReason::OddLot => write!(f, "not normalization lots"),
            CancelReason::OutOfPriceBand => write!(f, "Out Of PriceBand"),
            CancelReason::BrokenSession => write!(f, "Broken Session"),
            CancelReason::OutOfNormalTrading => write!(f, "Out Of NormalTrading"),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ImbalanceDirection {
    Buy,
    Sell,
    NoImbalance,
    InsufficientOrders,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CrossType {
    Opening,
    Closing,
    Halted,
    Intraday,
}

impl fmt::Display for CrossType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CrossType::Opening => write!(f, "Opening UnCross"),
            CrossType::Closing => write!(f, "Closing UnCross"),
            CrossType::Halted => write!(f, "UnCross after Halted"),
            CrossType::Intraday => write!(f, "Intraday UnCross"),
        }
    }
}
