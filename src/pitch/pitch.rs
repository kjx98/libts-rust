//! pitch - a parser for the SHFE Ponorama ITCH protocol 1.0
//!
//! It aims to sensibly handle the whole protocol.
//! It is zero-allocation and pretty fast. It will process
//! several million messages per second on a decent CPU.
//!
//!
//! The protocol specification can be found on the [SHFE website](http://www.shfe.comcn/PITCHSpecification.pdf)

use super::super::serde::{Error, Result};
use super::enums::*;
use super::proto::*;
use crate::{from_bytes as de_from_bytes, to_bytes as ser_to_bytes};
//use std::fmt;

/// An PITCH protocol message. Refer to the protocol spec for interpretation.
/// Message Type
///    pub tag: u8
#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    /// Integer identifying the underlying instrument updated daily
    pub index: u16,
    /// internal tracking number
    pub tracking: u16,
    /// Microseconds since Hours
    pub timestamp: u32,
    /// Body of one of the supported message types
    pub body: Body,
}

pub fn from_bytes(buf: &[u8]) -> Result<Message> {
    if buf.len() < 8 {
        return Err(Error::Eof);
    }
    match buf[0] {
        b'S' => {
            let r: SystemEventNet = de_from_bytes(buf)?;
            Ok(Message::from(r))
        }
        b'R' => {
            let r: &SymbolDirectoryNet = SymbolDirectoryNet::from_bytes(buf)?;
            Ok(Message::from(*r))
        }
        b'H' => {
            let r: SymbolTradingActionNet = de_from_bytes(buf)?;
            Ok(Message::from(r))
        }
        b'A' => {
            let r: AddOrderNet = de_from_bytes(buf)?;
            Ok(Message::from(r))
        }
        b'E' => {
            let r: OrderExecutedNet = de_from_bytes(buf)?;
            Ok(Message::from(r))
        }
        b'C' => {
            let r: OrderExecutedWithPriceNet = de_from_bytes(buf)?;
            Ok(Message::from(r))
        }
        b'X' => {
            let r: OrderCancelNet = de_from_bytes(buf)?;
            Ok(Message::from(r))
        }
        b'D' => {
            let r: OrderDeleteNet = de_from_bytes(buf)?;
            Ok(Message::from(r))
        }
        b'U' => {
            let r: OrderReplaceNet = de_from_bytes(buf)?;
            Ok(Message::from(r))
        }
        b'P' => {
            let r: TradeNet = de_from_bytes(buf)?;
            Ok(Message::from(r))
        }
        b'Q' => {
            let r: CrossTradeNet = de_from_bytes(buf)?;
            Ok(Message::from(r))
        }
        _ => Err(Error::Syntax),
    }
}

pub fn to_bytes(v: &Message) -> Result<Vec<u8>> {
    let (index, tracking, timestamp) = (v.index, v.tracking, v.timestamp);
    match &v.body {
        Body::SystemEvent(s) => {
            let tag = b'S';
            let (event_code, time_hours) = (s.event as u8, s.time_hours);
            let src = SystemEventNet {
                tag,
                event_code,
                index,
                tracking,
                timestamp,
                time_hours,
            };
            ser_to_bytes(&src)
        }
        Body::SymbolDirectory(s) => {
            let tag = b'R';
            let (
                market_category,
                symb,
                classification,
                precision,
                lot_size,
                turnover_multi,
                lower_limit,
                upper_limit,
            ) = (
                s.market_category,
                s.symbol.as_bytes(),
                s.classification,
                s.precision,
                s.round_lot_size,
                s.turnover_multi,
                s.lower_limit,
                s.upper_limit,
            );
            let mut symbol: [u8; 16] = Default::default();
            let ll = symb.len();
            if ll > 0 {
                let ll = if ll > 16 { 16 } else { ll };
                symbol[..ll].copy_from_slice(&symb);
            }
            let src = SymbolDirectoryNet {
                tag,
                index,
                tracking,
                timestamp,
                market_category,
                symbol,
                classification,
                precision,
                lot_size,
                turnover_multi,
                lower_limit,
                upper_limit,
            };
            SymbolDirectoryNet::to_bytes(&src)
        }
        Body::TradingAction(s) => {
            let tag = b'H';
            let (trading_state, reason) = (s.trading_state as u8, s.reason);
            let src = SymbolTradingActionNet {
                tag,
                index,
                tracking,
                timestamp,
                trading_state,
                reason,
            };
            ser_to_bytes(&src)
        }
        Body::AddOrder(s) => {
            let tag = b'A';
            let (buy_sell, ref_no) = (s.side as u8, s.reference);
            let (qty, price) = (s.qty, s.price);
            let src = AddOrderNet {
                tag,
                index,
                tracking,
                timestamp,
                buy_sell,
                ref_no,
                qty,
                price,
            };
            ser_to_bytes(&src)
        }
        Body::OrderExecuted(s) => {
            let tag = b'E';
            let (printable, ref_no) = (s.printable, s.reference);
            let (qty, match_no) = (s.qty, s.match_no);
            let src = OrderExecutedNet {
                tag,
                index,
                tracking,
                timestamp,
                printable,
                ref_no,
                qty,
                match_no,
            };
            ser_to_bytes(&src)
        }
        Body::OrderExecutedWithPrice(s) => {
            let tag = b'C';
            let (printable, ref_no, qty, price, match_no) =
                (s.printable, s.reference, s.qty, s.price, s.match_no);
            let src = OrderExecutedWithPriceNet {
                tag,
                index,
                tracking,
                timestamp,
                printable,
                ref_no,
                qty,
                price,
                match_no,
            };
            ser_to_bytes(&src)
        }
        Body::OrderCancelled(s) => {
            let tag = b'X';
            let cancel_reason = s.reason as u8;
            let (ref_no, qty) = (s.reference, s.cancelled);
            let src = OrderCancelNet {
                tag,
                index,
                tracking,
                timestamp,
                cancel_reason,
                ref_no,
                qty,
            };
            ser_to_bytes(&src)
        }
        Body::OrderDelete(s) => {
            let tag = b'D';
            let (cancel_reason, ref_no) = (s.reason as u8, s.reference);
            let src = OrderDeleteNet {
                tag,
                index,
                tracking,
                timestamp,
                cancel_reason,
                ref_no,
            };
            ser_to_bytes(&src)
        }
        Body::ReplaceOrder(s) => {
            let tag = b'U';
            let (ref_no, new_ref_no, qty, price) =
                (s.old_reference, s.new_reference, s.qty, s.price);
            let src = OrderReplaceNet {
                tag,
                index,
                tracking,
                timestamp,
                ref_no,
                new_ref_no,
                qty,
                price,
            };
            ser_to_bytes(&src)
        }
        Body::Trade(s) => {
            let tag = b'P';
            let (buy_sell, ref_no, qty, price, match_no) =
                (s.side as u8, s.reference, s.qty, s.price, s.match_no);
            let src = TradeNet {
                tag,
                index,
                tracking,
                timestamp,
                buy_sell,
                ref_no,
                qty,
                price,
                match_no,
            };
            ser_to_bytes(&src)
        }
        Body::CrossTrade(s) => {
            let tag = b'Q';
            let (qty, price) = (s.qty, s.price);
            let (match_no, type_) = (s.match_no, s.cross_type as u8);
            let (pclose, open_interest) = (s.pclose, s.open_interest);
            let src = CrossTradeNet {
                tag,
                index,
                tracking,
                timestamp,
                qty,
                price,
                match_no,
                type_,
                pclose,
                open_interest,
            };
            ser_to_bytes(&src)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SystemEvent {
    // System Event code
    pub event: EventCode,
    // hours since Unix Epoch
    pub time_hours: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolDirectory {
    pub symbol: String,
    pub market_category: u8, //MarketCategory,
    pub classification: u8,  //IssueClassification,
    pub precision: i8,
    pub round_lot_size: u32,
    pub turnover_multi: u32,
    pub lower_limit: i32,
    pub upper_limit: i32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TradingAction {
    pub trading_state: TradingState,
    pub reason: u16, //String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AddOrder {
    pub reference: u64,
    pub side: Side,
    pub qty: u32,
    pub price: i32,
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
    pub qty: u32,
    pub price: i32,
    pub match_no: u64,
    pub cross_type: CrossType,
    pub pclose: i32,
    pub open_interest: u32,
}

impl From<SystemEventNet> for Message {
    fn from(s: SystemEventNet) -> Message {
        let (index, tracking, timestamp) = (s.index, s.tracking, s.timestamp);
        let event = s.event();
        let time_hours = s.time_hours;
        let body = Body::SystemEvent(SystemEvent { event, time_hours });
        Message {
            index,
            tracking,
            timestamp,
            body,
        }
    }
}

impl From<SymbolDirectoryNet> for Message {
    fn from(s: SymbolDirectoryNet) -> Message {
        let (index, tracking, timestamp) = (s.index, s.tracking, s.timestamp);
        let (market_category, symbol, classification, precision) =
            (s.market_category, &s.symbol, s.classification, s.precision);
        let (round_lot_size, turnover_multi, lower_limit, upper_limit) =
            (s.lot_size, s.turnover_multi, s.lower_limit, s.upper_limit);
        let mut ll = 16;
        while ll > 0 {
            if symbol[ll - 1] != 0 {
                break;
            }
            ll -= 1
        }
        let res = unsafe { std::str::from_utf8_unchecked(&s.symbol[..ll]) };
        let symbol = res.to_owned();
        let body = Body::SymbolDirectory(SymbolDirectory {
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
            index,
            tracking,
            timestamp,
            body,
        }
    }
}

impl From<SymbolTradingActionNet> for Message {
    fn from(s: SymbolTradingActionNet) -> Message {
        let (index, tracking, timestamp) = (s.index, s.tracking, s.timestamp);
        let trading_state = s.state();
        let reason = s.reason;
        let body = Body::TradingAction(TradingAction {
            trading_state,
            reason,
        });
        Message {
            index,
            tracking,
            timestamp,
            body,
        }
    }
}

impl From<AddOrderNet> for Message {
    fn from(s: AddOrderNet) -> Message {
        let (index, tracking, timestamp) = (s.index, s.tracking, s.timestamp);
        let (reference, qty, price) = (s.ref_no, s.qty, s.price);
        let side = s.side();
        let body = Body::AddOrder(AddOrder {
            reference,
            side,
            qty,
            price,
        });
        Message {
            index,
            tracking,
            timestamp,
            body,
        }
    }
}

impl From<OrderExecutedNet> for Message {
    fn from(s: OrderExecutedNet) -> Message {
        let (index, tracking, timestamp) = (s.index, s.tracking, s.timestamp);
        let (printable, reference, qty, match_no) = (s.printable, s.ref_no, s.qty, s.match_no);
        let body = Body::OrderExecuted(OrderExecuted {
            printable,
            reference,
            qty,
            match_no,
        });
        Message {
            index,
            tracking,
            timestamp,
            body,
        }
    }
}

impl From<OrderExecutedWithPriceNet> for Message {
    fn from(s: OrderExecutedWithPriceNet) -> Message {
        let (index, tracking, timestamp) = (s.index, s.tracking, s.timestamp);
        let (printable, reference, qty, match_no, price) =
            (s.printable, s.ref_no, s.qty, s.match_no, s.price);
        let body = Body::OrderExecutedWithPrice(OrderExecutedWithPrice {
            printable,
            reference,
            qty,
            match_no,
            price,
        });
        Message {
            index,
            tracking,
            timestamp,
            body,
        }
    }
}

impl From<OrderCancelNet> for Message {
    fn from(s: OrderCancelNet) -> Message {
        let (index, tracking, timestamp) = (s.index, s.tracking, s.timestamp);
        let (reason, reference, cancelled) = (s.reason(), s.ref_no, s.qty);
        let body = Body::OrderCancelled(OrderCancelled {
            reason,
            reference,
            cancelled,
        });
        Message {
            index,
            tracking,
            timestamp,
            body,
        }
    }
}

impl From<OrderDeleteNet> for Message {
    fn from(s: OrderDeleteNet) -> Message {
        let (index, tracking, timestamp) = (s.index, s.tracking, s.timestamp);
        let (reason, reference) = (s.reason(), s.ref_no);
        let body = Body::OrderDelete(OrderDelete { reason, reference });
        Message {
            index,
            tracking,
            timestamp,
            body,
        }
    }
}

impl From<OrderReplaceNet> for Message {
    fn from(s: OrderReplaceNet) -> Message {
        let (index, tracking, timestamp) = (s.index, s.tracking, s.timestamp);
        let (qty, price) = (s.qty, s.price);
        let (old_reference, new_reference) = (s.ref_no, s.new_ref_no);
        let body = Body::ReplaceOrder(ReplaceOrder {
            old_reference,
            new_reference,
            qty,
            price,
        });
        Message {
            index,
            tracking,
            timestamp,
            body,
        }
    }
}

impl From<TradeNet> for Message {
    fn from(s: TradeNet) -> Message {
        let (index, tracking, timestamp) = (s.index, s.tracking, s.timestamp);
        let (reference, qty, price, match_no) = (s.ref_no, s.qty, s.price, s.match_no);
        let side = s.side();
        let body = Body::Trade(Trade {
            reference,
            side,
            qty,
            price,
            match_no,
        });
        Message {
            index,
            tracking,
            timestamp,
            body,
        }
    }
}

impl From<CrossTradeNet> for Message {
    fn from(s: CrossTradeNet) -> Message {
        let (index, tracking, timestamp) = (s.index, s.tracking, s.timestamp);
        let (qty, price, match_no) = (s.qty, s.price, s.match_no);
        let cross_type = s.cross_type();
        let (pclose, open_interest) = (s.pclose, s.open_interest);
        let body = Body::CrossTrade(CrossTrade {
            qty,
            price,
            match_no,
            cross_type,
            pclose,
            open_interest,
        });
        Message {
            index,
            tracking,
            timestamp,
            body,
        }
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
    ReplaceOrder(ReplaceOrder),
    Trade(Trade),
    CrossTrade(CrossTrade),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bytes() {
        let msg = Message {
            index: 1,
            tracking: 2,
            timestamp: 123456123,
            body: Body::AddOrder(AddOrder {
                reference: 202207041518,
                side: Side::Buy,
                qty: 100,
                price: 51050,
            }),
        };
        let expected: Vec<u8> = vec![
            b'A', b'B', 1, 0, 2, 0, 123, 202, 91, 7, 238, 151, 122, 20, 47, 0, 0, 0, 100, 0, 0, 0,
            106, 199, 0, 0,
        ];
        let bb = to_bytes(&msg).unwrap();
        assert_eq!(bb, expected);
    }

    #[test]
    fn test_from_bytes() {
        let expected = Message {
            index: 1,
            tracking: 2,
            timestamp: 123456123,
            body: Body::AddOrder(AddOrder {
                reference: 202207041518,
                side: Side::Buy,
                qty: 100,
                price: 51050,
            }),
        };
        let buf: Vec<u8> = vec![
            b'A', b'B', 1, 0, 2, 0, 123, 202, 91, 7, 238, 151, 122, 20, 47, 0, 0, 0, 100, 0, 0, 0,
            106, 199, 0, 0,
        ];
        let msg: Message = from_bytes(&buf[..]).unwrap();
        assert_eq!(msg, expected);
        let buf: Vec<u8> = vec![
            b'R', 78, 99, 117, 49, 57, 48, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 70, 0, 2, 0, 3, 0, 98,
            116, 140, 58, 5, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let _msg: Message = from_bytes(&buf[..]).unwrap();
    }
}
