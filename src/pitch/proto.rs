use super::enums::{CancelReason, CrossType, EventCode, Side, TradingState};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct SystemEventNet {
    pub tag: u8,
    pub event_code: u8,
    pub index: u16,
    pub tracking: u16,
    pub time_hours: u32,
    pub timestamp: u32,
}

impl SystemEventNet {
    pub fn event(&self) -> EventCode {
        match self.event_code {
            b'O' => EventCode::StartOfMessages,
            b'S' => EventCode::StartOfSystemHours,
            b'Q' => EventCode::StartOfMarketHours,
            b'M' => EventCode::EndOfMarketHours,
            b'E' => EventCode::EndOfSystemHours,
            b'C' => EventCode::EndOfMessages,
            b'A' => EventCode::EmergencyHalt,
            b'R' => EventCode::EmergencyQuoteOnly,
            b'B' => EventCode::EmergencyResumption,
            _ => todo!(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct SymbolDirectoryNet<'a> {
    pub tag: u8,
    pub market_category: u8,
    pub contract: &'a str,
    pub classification: u8,
    pub precision: u8,
    pub index: u16,
    pub tracking: u16,
    pub timestamp: u32,
    pub lot_size: u32,
    pub turnover_multi: u32,
    pub lower_limit: i32,
    pub upper_limit: i32,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct SymbolTradingActionNet {
    pub tag: u8,
    pub trading_state: u8,
    pub reason: u16,
    pub index: u16,
    pub tracking: u16,
    pub timestamp: u32,
}

impl SymbolTradingActionNet {
    pub fn state(&self) -> TradingState {
        match self.trading_state {
            b'H' => TradingState::Halted,
            b'P' => TradingState::PreAuction,
            b'A' => TradingState::Auction,
            b'U' => TradingState::Paused,
            b'C' => TradingState::Trading,
            b'B' => TradingState::Break,
            _ => todo!(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct AddOrderNet {
    pub tag: u8,
    pub buy_sell: u8,
    pub index: u16,
    pub tracking: u16,
    pub timestamp: u32,
    pub ref_no: u64,
    pub qty: u32,
    pub price: i32,
}

fn bs_side(bs: u8) -> Side {
    match bs {
        b'B' => Side::Buy,
        b'S' => Side::Sell,
        b'C' => Side::BuyCover,
        b'O' => Side::SellClose,
        _ => todo!(),
    }
}

impl AddOrderNet {
    pub fn side(&self) -> Side {
        bs_side(self.buy_sell)
    }
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct OrderExecutedNet {
    pub tag: u8,
    pub printable: u8,
    pub index: u16,
    pub tracking: u16,
    pub timestamp: u32,
    pub ref_no: u64,
    pub qty: u32,
    pub match_no: u64,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct OrderEexecutedWithPriceNet {
    pub tag: u8,
    pub printable: u8,
    pub index: u16,
    pub tracking: u16,
    pub timestamp: u32,
    pub ref_no: u64,
    pub qty: u32,
    pub match_no: u64,
    pub price: i32,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct OrderCancelNet {
    pub tag: u8,
    pub cancel_reason: u8,
    pub index: u16,
    pub tracking: u16,
    pub timestamp: u32,
    pub ref_no: u64,
    pub qty: u32,
}

impl OrderCancelNet {
    pub fn reason(&self) -> CancelReason {
        cancel_reason(self.cancel_reason)
    }
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct OrderDeleteNet {
    pub tag: u8,
    pub cancel_reason: u8,
    pub index: u16,
    pub tracking: u16,
    pub timestamp: u32,
    pub ref_no: u64,
}

impl OrderDeleteNet {
    pub fn reason(&self) -> CancelReason {
        cancel_reason(self.cancel_reason)
    }
}

fn cancel_reason(r: u8) -> CancelReason {
    match r {
        b'U' => CancelReason::ByUser,
        b'A' => CancelReason::Arb,
        b'M' => CancelReason::ByModifyOrder,
        b'O' => CancelReason::OddLot,
        b'B' => CancelReason::OutOfPriceBand,
        b'S' => CancelReason::BrokenSession,
        b'N' => CancelReason::OutOfNormalTrading,
        _ => todo!(),
    }
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct OrderReplaceNet {
    pub tag: u8,
    pub index: u16,
    pub tracking: u16,
    pub timestamp: u32,
    pub ref_no: u64,
    pub new_ref_no: u64,
    pub qty: u32,
    pub price: i32,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct TradeNet {
    pub tag: u8,
    pub buy_sell: u8,
    pub index: u16,
    pub tracking: u16,
    pub timestamp: u32,
    pub ref_no: u64,
    pub qty: u32,
    pub price: i32,
    pub match_no: u64,
}

impl TradeNet {
    pub fn side(&self) -> Side {
        bs_side(self.buy_sell)
    }
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct CrossTradeNet {
    pub tag: u8,
    pub type_: u8,
    pub index: u16,
    pub tracking: u16,
    pub timestamp: u32,
    pub qty: u32,
    pub price: i32,
    pub pclose: i32,
    pub open_interest: u32,
    pub match_no: u64,
}

impl CrossTradeNet {
    pub fn cross_type(&self) -> CrossType {
        match self.type_ {
            b'O' => CrossType::Opening,
            b'C' => CrossType::Closing,
            b'H' => CrossType::Halted,
            b'I' => CrossType::Intraday,
            _ => CrossType::Opening,
        }
    }
}
