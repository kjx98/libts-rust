//! pitch - a parser for the SHFE Ponorama ITCH protocol 1.0
//!
//! It aims to sensibly handle the whole protocol.
//! It is zero-allocation and pretty fast. It will process
//! several million messages per second on a decent CPU.
//!
//!
//! The protocol specification can be found on the [SHFE website](http://www.shfe.comcn/PITCHSpecification.pdf)

use super::enums::*;
use super::proto::*;
//use std::fmt;

/// An PITCH protocol message. Refer to the protocol spec for interpretation.
#[derive(Debug, Clone, PartialEq)]
pub struct Message<'a> {
    /// Message Type
    pub tag: u8,
    /// Integer identifying the underlying instrument updated daily
    pub index: u16,
    /// internal tracking number
    pub tracking: u16,
    /// Microseconds since Hours
    pub timestamp: u32,
    /// Body of one of the supported message types
    pub body: Body<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SystemEvent {
    // System Event code
    pub event: EventCode,
    // hours since Unix Epoch
    pub time_hours: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TradingAction {
    pub trading_state: TradingState,
    pub reason: u16, //String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrderExecuted {
    pub printable: bool,
    pub reference: u64,
    pub qty: u32, // qty executed
    pub match_no: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrderExecutedWithPrice {
    pub printable: bool,
    pub reference: u64,
    pub qty: u32, // qty executed
    pub price: i32,
    pub match_no: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrderCancelled {
    pub reason: CancelReason,
    pub reference: u64,
    pub cancelled: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrderDelete {
    pub reason: CancelReason,
    pub reference: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolDirectory<'a> {
    pub symbol: &'a str,
    pub market_category: u8, //MarketCategory,
    pub classification: u8,  //IssueClassification,
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
pub struct Trade {
    pub reference: u64,
    pub side: Side,
    pub qty: u32,
    pub price: i32,
    pub match_no: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CrossTrade {
    pub qty: u64,
    pub price: i32,
    pub match_no: u64,
    pub cross_type: CrossType,
}

impl<'a> From<SystemEventNet> for Message<'a> {
    fn from(s: SystemEventNet) -> Message<'a> {
        let (tag, index, tracking, timestamp) = (s.tag, s.index, s.tracking, s.timestamp);
        let event = s.event();
        let time_hours = s.time_hours;
        let body = Body::<'a>::SystemEvent(SystemEvent { event, time_hours });
        Message {
            tag,
            index,
            tracking,
            timestamp,
            body,
        }
    }
}

impl<'a> From<SymbolDirectoryNet<'a>> for Message<'a> {
    fn from(s: SymbolDirectoryNet<'a>) -> Message<'a> {
        let (tag, index, tracking, timestamp) = (s.tag, s.index, s.tracking, s.timestamp);
        let (market_category, symbol, classification, precision) =
            (s.market_category, s.symbol, s.classification, s.precision);
        let (round_lot_size, turnover_multi, lower_limit, upper_limit) =
            (s.lot_size, s.turnover_multi, s.lower_limit, s.upper_limit);
        let body = Body::<'a>::SymbolDirectory(SymbolDirectory {
            symbol,
            market_category,
            classification,
            precision,
            round_lot_size,
            turnover_multi,
            lower_limit,
            upper_limit,
        });
        Message {
            tag,
            index,
            tracking,
            timestamp,
            body,
        }
    }
}

impl<'a> From<SymbolTradingActionNet> for Message<'a> {
    fn from(s: SymbolTradingActionNet) -> Message<'a> {
        let (tag, index, tracking, timestamp) = (s.tag, s.index, s.tracking, s.timestamp);
        let trading_state = s.state();
        let reason = s.reason;
        let body = Body::<'a>::TradingAction(TradingAction {
            trading_state,
            reason,
        });
        Message {
            tag,
            index,
            tracking,
            timestamp,
            body,
        }
    }
}

impl<'a> From<AddOrderNet> for Message<'a> {
    fn from(s: AddOrderNet) -> Message<'a> {
        let (tag, index, tracking, timestamp) = (s.tag, s.index, s.tracking, s.timestamp);
        let (reference, qty, price) = (s.ref_no, s.qty, s.price);
        let side = s.side();
        let body = Body::<'a>::AddOrder(AddOrder {
            reference,
            side,
            qty,
            price,
        });
        Message {
            tag,
            index,
            tracking,
            timestamp,
            body,
        }
    }
}

impl<'a> From<OrderExecutedNet> for Message<'a> {
    fn from(s: OrderExecutedNet) -> Message<'a> {
        let (tag, index, tracking, timestamp) = (s.tag, s.index, s.tracking, s.timestamp);
        let (printable, reference, qty, match_no) = (s.printable, s.ref_no, s.qty, s.match_no);
        let body = Body::<'a>::OrderExecuted(OrderExecuted {
            printable,
            reference,
            qty,
            match_no,
        });
        Message {
            tag,
            index,
            tracking,
            timestamp,
            body,
        }
    }
}

impl<'a> From<OrderExecutedWithPriceNet> for Message<'a> {
    fn from(s: OrderExecutedWithPriceNet) -> Message<'a> {
        let (tag, index, tracking, timestamp) = (s.tag, s.index, s.tracking, s.timestamp);
        let (printable, reference, qty, match_no, price) =
            (s.printable, s.ref_no, s.qty, s.match_no, s.price);
        let body = Body::<'a>::OrderExecutedWithPrice(OrderExecutedWithPrice {
            printable,
            reference,
            qty,
            match_no,
            price,
        });
        Message {
            tag,
            index,
            tracking,
            timestamp,
            body,
        }
    }
}

impl<'a> From<OrderReplaceNet> for Message<'a> {
    fn from(s: OrderReplaceNet) -> Message<'a> {
        let (tag, index, tracking, timestamp) = (s.tag, s.index, s.tracking, s.timestamp);
        let (qty, price) = (s.qty, s.price);
        let (old_reference, new_reference) = (s.ref_no, s.new_ref_no);
        let body = Body::<'a>::ReplaceOrder(ReplaceOrder {
            old_reference,
            new_reference,
            qty,
            price,
        });
        Message {
            tag,
            index,
            tracking,
            timestamp,
            body,
        }
    }
}

/// The message body. Refer to the protocol spec for interpretation.
#[derive(Debug, Clone, PartialEq)]
pub enum Body<'a> {
    SystemEvent(SystemEvent),
    SymbolDirectory(SymbolDirectory<'a>),
    TradingAction(TradingAction),
    AddOrder(AddOrder),
    OrderExecuted(OrderExecuted),
    OrderExecutedWithPrice(OrderExecutedWithPrice),
    OrderCancelled(OrderCancelled),
    OrderDelete(OrderDelete),
    ReplaceOrder(ReplaceOrder),
    Trade(Trade),
    CrossTrade(CrossTrade),
}
