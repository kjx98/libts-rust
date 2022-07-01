use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct SystemEventNet {
    pub msg_type: u8,
    pub event_code: u8,
    pub index: u16,
    pub tracking: u16,
    pub time_hours: u32,
    pub timestamp: u32,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct SymbolDirectoryNet<'a> {
    pub msg_type: u8,
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
    pub msg_type: u8,
    pub trading_state: u8,
    pub reason: u16,
    pub index: u16,
    pub tracking: u16,
    pub timestamp: u32,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct AddOrderNet {
    pub msg_type: u8,
    pub buy_sell: u8,
    pub index: u16,
    pub tracking: u16,
    pub timestamp: u32,
    pub ref_no: u64,
    pub qty: u32,
    pub price: i32,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct OrderExecutedNet {
    pub msg_type: u8,
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
    pub msg_type: u8,
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
    pub msg_type: u8,
    pub cancel_reason: u8,
    pub index: u16,
    pub tracking: u16,
    pub timestamp: u32,
    pub ref_no: u64,
    pub qty: u32,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct OrderDeleteNet {
    pub msg_type: u8,
    pub cancel_reason: u8,
    pub index: u16,
    pub tracking: u16,
    pub timestamp: u32,
    pub ref_no: u64,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct OrderReplaceNet {
    pub msg_type: u8,
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
    pub msg_type: u8,
    pub buy_sell: u8,
    pub index: u16,
    pub tracking: u16,
    pub timestamp: u32,
    pub ref_no: u64,
    pub qty: u32,
    pub price: i32,
    pub match_no: u64,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq)]
pub struct CrossTradeNet {
    pub msg_type: u8,
    pub cross_type: u8,
    pub index: u16,
    pub tracking: u16,
    pub timestamp: u32,
    pub qty: u32,
    pub price: i32,
    pub pclose: i32,
    pub open_interest: u32,
    pub match_no: u64,
}
