use crate::Balance;

pub const CENTS: Balance = 1_000 * MILLICENTS;
pub const MILLICENTS: Balance = 1_000_000_000;

pub const fn deposit(items: u32, bytes: u32) -> Balance {
  items as Balance * 15 * CENTS + (bytes as Balance) * 6 * CENTS
}
