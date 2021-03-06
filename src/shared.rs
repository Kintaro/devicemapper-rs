// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// A module to contain functionality shared among the various types of
/// devices.

use super::consts::{DmFlags, DM_SUSPEND};
use super::deviceinfo::DeviceInfo;
use super::dm::{DevId, DM};
use super::result::DmResult;
use super::types::TargetLineArg;

/// Load the table for a device.
pub fn table_load<T1, T2>(dm: &DM,
                          id: &DevId,
                          table: &[TargetLineArg<T1, T2>])
                          -> DmResult<DeviceInfo>
    where T1: AsRef<str>,
          T2: AsRef<str>
{
    let dev_info = try!(dm.table_load(id, table));
    try!(dm.device_suspend(id, DmFlags::empty()));
    Ok(dev_info)

}

/// Reload the table for a device
pub fn table_reload<T1, T2>(dm: &DM,
                            id: &DevId,
                            table: &[TargetLineArg<T1, T2>])
                            -> DmResult<DeviceInfo>
    where T1: AsRef<str>,
          T2: AsRef<str>
{
    let dev_info = try!(dm.table_load(id, table));
    try!(dm.device_suspend(id, DM_SUSPEND));
    try!(dm.device_suspend(id, DmFlags::empty()));
    Ok(dev_info)
}

/// Check if a device of the given name exists.
pub fn device_exists(dm: &DM, name: &str) -> DmResult<bool> {
    Ok(try!(dm.list_devices()
                .map(|l| l.iter().any(|&(ref n, _)| n == name))))
}
