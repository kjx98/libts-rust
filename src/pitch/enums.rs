use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventCode {
    StartOfMessages = b'O' as isize,
    StartOfSystemHours = b'S' as isize,
    StartOfMarketHours = b'Q' as isize,
    EndOfMarketHours = b'M' as isize,
    EndOfSystemHours = b'E' as isize,
    EndOfMessages = b'C' as isize,
    EmergencyHalt = b'A' as isize,
    EmergencyQuoteOnly = b'R' as isize,
    EmergencyResumption = b'B' as isize,
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
    Shfe = b'H' as isize,
    Dce = b'D' as isize,
    Czce = b'C' as isize,
    Cffex = b'F' as isize,
    Gce = b'G' as isize,
    Sse = b'S' as isize,
    Szse = b'Z' as isize,
    Unavailable = 0u8 as isize,
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
pub enum MarketParticipantState {
    Active,
    Excused,
    Withdrawn,
    Suspended,
    Deleted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradingState {
    Halted = b'H' as isize,
    PreAuction = b'P' as isize,
    Auction = b'A' as isize,
    Paused = b'U' as isize,
    Trading = b'C' as isize,
    Break = b'B' as isize,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy = b'B' as isize,
    Sell = b'S' as isize,
    BuyCover = b'C' as isize,
    SellClose = b'O' as isize,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CancelReason {
    ByUser = b'U' as isize,
    Arb = b'A' as isize,
    ByModifyOrder = b'M' as isize,
    OddLot = b'O' as isize,
    OutOfPriceBand = b'B' as isize,
    BrokenSession = b'S' as isize,
    OutOfNormalTrading = b'N' as isize,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrossType {
    Opening = b'O' as isize,
    Closing = b'C' as isize,
    Halted = b'H' as isize,
    Intraday = b'I' as isize,
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
