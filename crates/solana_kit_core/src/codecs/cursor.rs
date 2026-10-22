//! Slice cursors and allocation budgets.

use super::Error;

/// Caller-selected limits for one decoding operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Limits {
    /// Maximum input size in bytes, checked before decoding starts.
    pub max_bytes: usize,
    /// Total output elements allocated by all nested decoders.
    pub max_items: usize,
}

impl Limits {
    /// Constructs explicit byte and element budgets.
    #[must_use]
    pub const fn new(max_bytes: usize, max_items: usize) -> Self {
        Self {
            max_bytes,
            max_items,
        }
    }
}

/// A forward-only reader whose slices and allocation claims are checked.
pub struct Reader<'a> {
    bytes: &'a [u8],
    position: usize,
    items_left: usize,
}

impl<'a> Reader<'a> {
    /// Starts a read, rejecting input larger than the byte budget.
    pub fn new(bytes: &'a [u8], limits: Limits) -> Result<Self, Error> {
        if bytes.len() > limits.max_bytes {
            return Err(Error::LimitExceeded);
        }

        Ok(Self {
            bytes,
            position: 0,
            items_left: limits.max_items,
        })
    }

    /// Returns the absolute byte offset.
    #[must_use]
    pub const fn position(&self) -> usize {
        self.position
    }

    /// Returns unread bytes without advancing.
    #[must_use]
    pub fn remaining(&self) -> &'a [u8] {
        &self.bytes[self.position..]
    }

    /// Borrows exactly `size` bytes, rejecting overflow and truncation.
    pub fn take(&mut self, size: usize) -> Result<&'a [u8], Error> {
        let end = self.position.checked_add(size).ok_or(Error::Overflow)?;
        let bytes = self.bytes.get(self.position..end).ok_or(Error::Truncated)?;
        self.position = end;
        Ok(bytes)
    }

    /// Claims elements before allocating; zero-size element codecs are bounded too.
    pub fn claim(&mut self, count: usize) -> Result<(), Error> {
        self.items_left = self
            .items_left
            .checked_sub(count)
            .ok_or(Error::LimitExceeded)?;
        Ok(())
    }

    /// Reads an exact frame with the same allocation budget; unread frame bytes
    /// are an error. The outer position advances even if the inner read fails.
    pub fn frame<T>(
        &mut self,
        size: usize,
        read: impl FnOnce(&mut Reader<'_>) -> Result<T, Error>,
    ) -> Result<T, Error> {
        let bytes = self.take(size)?;
        let mut inner = Reader {
            bytes,
            position: 0,
            items_left: self.items_left,
        };
        let result = read(&mut inner);
        self.items_left = inner.items_left;
        let value = result?;
        inner.finish()?;
        Ok(value)
    }

    /// Rejects any unread bytes.
    pub fn finish(&self) -> Result<(), Error> {
        if !self.remaining().is_empty() {
            return Err(Error::TrailingBytes);
        }

        Ok(())
    }
}

/// A forward-only writer into a fixed-size caller-owned slice.
pub struct Writer<'a> {
    bytes: &'a mut [u8],
    position: usize,
}

impl<'a> Writer<'a> {
    /// Starts writing at offset zero.
    #[must_use]
    pub fn new(bytes: &'a mut [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    /// Returns the next write offset.
    #[must_use]
    pub const fn position(&self) -> usize {
        self.position
    }

    /// Copies bytes after checking the entire destination range.
    pub fn put(&mut self, bytes: &[u8]) -> Result<(), Error> {
        let end = self
            .position
            .checked_add(bytes.len())
            .ok_or(Error::Overflow)?;
        let output = self
            .bytes
            .get_mut(self.position..end)
            .ok_or(Error::Truncated)?;
        output.copy_from_slice(bytes);
        self.position = end;
        Ok(())
    }

    /// Fills bytes after checking the entire destination range.
    pub fn fill(&mut self, size: usize, value: u8) -> Result<(), Error> {
        let end = self.position.checked_add(size).ok_or(Error::Overflow)?;
        let output = self
            .bytes
            .get_mut(self.position..end)
            .ok_or(Error::Truncated)?;
        output.fill(value);
        self.position = end;
        Ok(())
    }

    /// Restricts an inner write to exactly `size` bytes. A successful writer must
    /// fill the frame; a failed write may have changed bytes inside that frame.
    pub fn frame(
        &mut self,
        size: usize,
        write: impl FnOnce(&mut Writer<'_>) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let end = self.position.checked_add(size).ok_or(Error::Overflow)?;
        let output = self
            .bytes
            .get_mut(self.position..end)
            .ok_or(Error::Truncated)?;
        let mut inner = Writer::new(output);
        write(&mut inner)?;

        if inner.position() != size {
            return Err(Error::InvalidValue);
        }

        self.position = end;
        Ok(())
    }
}
