//! # cuid-rust
//!
//! CUID generation in rust
//!

#[macro_use]
extern crate lazy_static;
extern crate hostname;
extern crate parking_lot;
extern crate rand;

mod counter;
mod error;
mod fingerprint;
mod random;
mod text;
mod time;

pub use error::CuidError;

use parking_lot::Mutex;

static BASE: u8 = 36;
static BLOCK_SIZE: u8 = 4;
static DISCRETE_VALUES: u32 = 1679616; // BASE^BLOCK_SIZE
static START_STR: &str = "c";

lazy_static! {
    static ref FINGERPRINT: String = fingerprint::fingerprint().unwrap().into();
    static ref COUNTER: Mutex<u32> = Mutex::new(0);
}

/// Generate a CUID
///
/// # Examples
///
/// ```
/// extern crate cuid;
/// let id = cuid::cuid();
/// assert!(cuid::is_cuid(id.unwrap()));
/// ```
pub fn cuid() -> Result<String, CuidError> {
    Ok([
        START_STR,
        &time::timestamp()?,
        &counter::current(),
        &FINGERPRINT,
        &random::random_block()?,
        &random::random_block()?,
    ]
    .concat())
}

/// Generate a CUID slug
///
/// CUID slugs are shorter, appropriate for short URLs or other uses
/// where uniqueness across deployments is not the primary requirement.
///
/// # Examples
///
/// ```
/// extern crate cuid;
/// let slug = cuid::slug();
/// assert!(cuid::is_slug(slug.unwrap()));
/// ```
pub fn slug() -> Result<String, CuidError> {
    let timestamp = time::timestamp()?;
    let count = counter::current();
    let rand = random::random_block()?;
    Ok([
        &timestamp[timestamp.len() - 2..],
        &count[count.len().saturating_sub(4)..],
        &FINGERPRINT[..1],
        &FINGERPRINT[FINGERPRINT.len() - 1..],
        &rand[rand.len() - 2..],
    ]
    .concat())
}

/// Return whether a string is a legitimate CUID
///
/// # Examples
///
/// ```
/// extern crate cuid;
/// let id = cuid::cuid().unwrap();
/// assert!(cuid::is_cuid(id));
/// ```
pub fn is_cuid<S: Into<String>>(to_check: S) -> bool {
    &to_check.into()[..1] == START_STR
}

/// Return whether a string is a legitimate CUID slug
///
/// # Examples
///
/// ```
/// extern crate cuid;
/// let slug = cuid::slug().unwrap();
/// assert!(cuid::is_slug(slug));
/// ```
pub fn is_slug<S: Into<String>>(to_check: S) -> bool {
    let length = to_check.into().len();
    length >= 7 && length <= 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_discrete_values() {
        assert_eq!((BASE as u32).pow(BLOCK_SIZE as u32), DISCRETE_VALUES,);
    }

    #[test]
    fn cuid_is_cuid() {
        assert!(is_cuid(cuid().unwrap()));
    }

    #[test]
    fn slug_max_len() {
        assert!(slug().unwrap().len() <= 10);
    }

    #[test]
    fn slug_min_len() {
        assert!(slug().unwrap().len() >= 7);
    }

    #[test]
    fn slug_is_slug() {
        assert!(is_slug(slug().unwrap()));
    }

}
