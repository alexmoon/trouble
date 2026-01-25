//! UUID types.

use bt_hci::uuid::BluetoothUuid;

use crate::codec::{Encode, Error, Type};

/// Bluetooth UUID
pub type Uuid = BluetoothUuid;

impl Type for Uuid {
    fn size(&self) -> usize {
        self.as_le_slice().len()
    }
}

impl Encode for Uuid {
    fn encode(&self, dest: &mut [u8]) -> Result<(), Error> {
        dest.copy_from_slice(self.as_le_slice());
        Ok(())
    }
}
