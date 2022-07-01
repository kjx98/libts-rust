
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

/*
pub(crate) fn parse_issue_classification(input: &[u8]) -> IResult<&[u8], IssueClassification> {
    map_opt!(input, be_u8, |v| {
        use IssueClassification::*;
        Some(match v {
            b'A' => AmericanDepositaryShare,
            b'B' => Bond,
            b'C' => CommonStock,
            b'F' => DepositoryReceipt,
            b'I' => A144,
            b'L' => LimitedPartnership,
            b'N' => Notes,
            b'O' => OrdinaryShare,
            b'P' => PreferredStock,
            b'Q' => OtherSecurities,
            b'R' => Right,
            b'S' => SharesOfBeneficialInterest,
            b'T' => ConvertibleDebenture,
            b'U' => Unit,
            b'V' => UnitsPerBenifInt,
            b'W' => Warrant,
            _ => return None,
        })
    })
}
*/

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
