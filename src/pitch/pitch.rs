//! pitch - a parser for the SHFE Ponorama ITCH protocol 1.0
//!
//! It aims to sensibly handle the whole protocol.
//! It is zero-allocation and pretty fast. It will process
//! several million messages per second on a decent CPU.
//!
//!
//! The protocol specification can be found on the [SHFE website](http://www.shfe.comcn/PITCHSpecification.pdf)

use std::fmt;
use super::proto::*;


/// An PITCH protocol message. Refer to the protocol spec for interpretation.
#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    /// Message Type
    pub tag: u8,
    /// Integer identifying the underlying instrument updated daily
    pub index: u16,
    /// internal tracking number
    pub tracking: u16,
    /// Microseconds since Hours
    pub timestamp: u32,
    /// Body of one of the supported message types
    pub body: Body,
}

pub struct SystemEvent {
    // System Event code
    pub event: EventCode,
    // hours since Unix Epoch
    pub time_hours: u32,
}

pub struct TradingAction {
    pub trading_state: TradingState,
    pub reason: u16,    //String,
}

pub struct OrderExecuted {
    pub printable: bool,
    pub reference: u64,
    pub executed: u32,  // qty executed
    pub match_number: u64,
}

pub struct OrderExecutedWithPrice {
    pub printable: bool,
    pub reference: u64,
    pub executed: u32,  // qty executed
    pub price: i32,
    pub match_number: u64,
}

pub struct OrderCancelled {
    pub reason: CancelReason,
    pub reference: u64,
    pub cancelled: u32,
}

pub struct OrderDelete {
    pub reason: CancelReason,
    pub reference: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolDirectory<'a> {
    pub symbol: &'a str,
    pub market_category: MarketCategory,
    pub classification: IssueClassification,
    pub precision: u8,
    pub round_lot_size: u32,
    pub turnover_multi: u32,
    pub lower_limit: i32,
    pub upper_limit: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AddOrder {
    pub reference: u64,
    pub side: Side,
    pub qty: u32,
    pub price: i32,
}


#[derive(Debug, Clone, PartialEq)]
pub struct ReplaceOrder {
    pub old_reference: u64,
    pub new_reference: u64,
    pub qty: u32,
    pub price: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NonCrossTrade {
    pub reference: u64,
    pub side: Side,
    pub qty: u32,
    pub price: i32,
    pub match_no: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CrossTrade {
    pub shares: u64,
    pub stock: ArrayString8,
    pub cross_price: Price4,
    pub match_number: u64,
    pub cross_type: CrossType,
}


impl From<AddOrderNet> for Message {
    fn from(s: AddOrderNet) -> Message {
        let (tag, index, tracking, timestamp) = (s.tag, s.index, s.tracking, s.timestamp);
        let (reference, qty, price) = (s.ref_no, s.qty, s.price);
        let side = s.side();
        let body = Body::AddOrder(AddOrder{reference, side, qty, price});
        Message{ tag, index, tracking, timestamp, body }
    }
}

impl From<OrderReplaceNet> for Message {
    fn from(s: OrderReplaceNet) -> Message {
        let (tag, index, tracking, timestamp) = (s.tag, s.index, s.tracking, s.timestamp);
        let (qty, price) = (s.qty, s.price);
        let (old_reference, new_reference) = (s.ref_no, s.new_ref_no);
        let body = Body::ReplaceOrder(ReplaceOrder{old_reference, new_reference, qty, price});
        Message{ tag, index, tracking, timestamp, body }
    }
}

impl From<SystemEventNet> for Message {
    fn from(s: SystemEventNet) -> Message {
        let (tag, index, tracking, timestamp) = (s.tag, s.index, s.tracking, s.timestamp);
        let event = s.event();
        let time_hours = s.time_hours;
        let body = Body::SystemEvent(SystemEvent{ event, time_hours });
        Message{ tag, index, tracking, timestamp, body }
    }
}

impl From<SymbolTradingActionNet> for Message {
    fn from(s: SymbolTradingActionNet) -> Message {
        let (tag, index, tracking, timestamp) = (s.tag, s.index, s.tracking, s.timestamp);
        let trading_state = s.state();
        let reason = s.reason;
        let body = Body::TradingAction(TradingAction{trading_state, reason});
        Message{ tag, index, tracking, timestamp, body }
    }
}


/// The message body. Refer to the protocol spec for interpretation.
#[derive(Debug, Clone, PartialEq)]
pub enum Body {
    SystemEvent(SystemEvent),
    SymbolDirectory(SymbolDirectory),
    TradingAction(TradingAction),
    AddOrder(AddOrder),
    OrderExecuted(OrderExecuted),
    OrderExecutedWithPrice(OrderExecutedWithPrice),
    OrderCancelled(OrderCancelled),
    OrderDelete(OrderDelete),
    CrossTrade(CrossTrade),
    ReplaceOrder(ReplaceOrder),
}
