#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    revision: Revision,
    config_0: Config0,
    bluetooth_transmit: BluetoothTransmit,
    bt_tx_abort: BtTxAbort,
    bluetooth_receive: BluetoothReceive,
    bt_rx_abort: BtRxAbort,
    wifi_transmit: WifiTransmit,
    wlan_tx_abort: WlanTxAbort,
    wifi_receive: WifiReceive,
    wlan_rx_abort: WlanRxAbort,
    config_2: Config2,
}
impl RegisterBlock {
    #[doc = "0x400 - Peripheral silicon revision"]
    #[inline(always)]
    pub const fn revision(&self) -> &Revision {
        &self.revision
    }
    #[doc = "0x404 - Arbitration configuration register 0"]
    #[inline(always)]
    pub const fn config_0(&self) -> &Config0 {
        &self.config_0
    }
    #[doc = "0x408 - Bluetooth transmit configuration"]
    #[inline(always)]
    pub const fn bluetooth_transmit(&self) -> &BluetoothTransmit {
        &self.bluetooth_transmit
    }
    #[doc = "0x40c - Bluetooth transmit abort configuration"]
    #[inline(always)]
    pub const fn bt_tx_abort(&self) -> &BtTxAbort {
        &self.bt_tx_abort
    }
    #[doc = "0x410 - Bluetooth receive configuration"]
    #[inline(always)]
    pub const fn bluetooth_receive(&self) -> &BluetoothReceive {
        &self.bluetooth_receive
    }
    #[doc = "0x414 - Bluetooth receive abort configuration"]
    #[inline(always)]
    pub const fn bt_rx_abort(&self) -> &BtRxAbort {
        &self.bt_rx_abort
    }
    #[doc = "0x418 - WiFi transmit configuration"]
    #[inline(always)]
    pub const fn wifi_transmit(&self) -> &WifiTransmit {
        &self.wifi_transmit
    }
    #[doc = "0x41c - WLAN transmit abort configuration"]
    #[inline(always)]
    pub const fn wlan_tx_abort(&self) -> &WlanTxAbort {
        &self.wlan_tx_abort
    }
    #[doc = "0x420 - WiFi receive configuration"]
    #[inline(always)]
    pub const fn wifi_receive(&self) -> &WifiReceive {
        &self.wifi_receive
    }
    #[doc = "0x424 - WLAN receive abort configuration"]
    #[inline(always)]
    pub const fn wlan_rx_abort(&self) -> &WlanRxAbort {
        &self.wlan_rx_abort
    }
    #[doc = "0x428 - Arbitration configuration register 2 (PTA_REG_CONFIG2)"]
    #[inline(always)]
    pub const fn config_2(&self) -> &Config2 {
        &self.config_2
    }
}
#[doc = "revision (r) register accessor: Peripheral silicon revision\n\nYou can [`read`](crate::Reg::read) this register and get [`revision::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@revision`] module"]
#[doc(alias = "revision")]
pub type Revision = crate::Reg<revision::RevisionSpec>;
#[doc = "Peripheral silicon revision"]
pub mod revision;
#[doc = "config_0 (rw) register accessor: Arbitration configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`config_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_0`] module"]
#[doc(alias = "config_0")]
pub type Config0 = crate::Reg<config_0::Config0Spec>;
#[doc = "Arbitration configuration register 0"]
pub mod config_0;
#[doc = "bluetooth_transmit (rw) register accessor: Bluetooth transmit configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`bluetooth_transmit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bluetooth_transmit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bluetooth_transmit`] module"]
#[doc(alias = "bluetooth_transmit")]
pub type BluetoothTransmit = crate::Reg<bluetooth_transmit::BluetoothTransmitSpec>;
#[doc = "Bluetooth transmit configuration"]
pub mod bluetooth_transmit;
#[doc = "bt_tx_abort (rw) register accessor: Bluetooth transmit abort configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`bt_tx_abort::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bt_tx_abort::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bt_tx_abort`] module"]
#[doc(alias = "bt_tx_abort")]
pub type BtTxAbort = crate::Reg<bt_tx_abort::BtTxAbortSpec>;
#[doc = "Bluetooth transmit abort configuration"]
pub mod bt_tx_abort;
#[doc = "bluetooth_receive (rw) register accessor: Bluetooth receive configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`bluetooth_receive::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bluetooth_receive::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bluetooth_receive`] module"]
#[doc(alias = "bluetooth_receive")]
pub type BluetoothReceive = crate::Reg<bluetooth_receive::BluetoothReceiveSpec>;
#[doc = "Bluetooth receive configuration"]
pub mod bluetooth_receive;
#[doc = "bt_rx_abort (rw) register accessor: Bluetooth receive abort configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`bt_rx_abort::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bt_rx_abort::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bt_rx_abort`] module"]
#[doc(alias = "bt_rx_abort")]
pub type BtRxAbort = crate::Reg<bt_rx_abort::BtRxAbortSpec>;
#[doc = "Bluetooth receive abort configuration"]
pub mod bt_rx_abort;
#[doc = "wifi_transmit (rw) register accessor: WiFi transmit configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_transmit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_transmit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_transmit`] module"]
#[doc(alias = "wifi_transmit")]
pub type WifiTransmit = crate::Reg<wifi_transmit::WifiTransmitSpec>;
#[doc = "WiFi transmit configuration"]
pub mod wifi_transmit;
#[doc = "wlan_tx_abort (rw) register accessor: WLAN transmit abort configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`wlan_tx_abort::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wlan_tx_abort::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wlan_tx_abort`] module"]
#[doc(alias = "wlan_tx_abort")]
pub type WlanTxAbort = crate::Reg<wlan_tx_abort::WlanTxAbortSpec>;
#[doc = "WLAN transmit abort configuration"]
pub mod wlan_tx_abort;
#[doc = "wifi_receive (rw) register accessor: WiFi receive configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`wifi_receive::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wifi_receive::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wifi_receive`] module"]
#[doc(alias = "wifi_receive")]
pub type WifiReceive = crate::Reg<wifi_receive::WifiReceiveSpec>;
#[doc = "WiFi receive configuration"]
pub mod wifi_receive;
#[doc = "wlan_rx_abort (rw) register accessor: WLAN receive abort configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`wlan_rx_abort::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wlan_rx_abort::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wlan_rx_abort`] module"]
#[doc(alias = "wlan_rx_abort")]
pub type WlanRxAbort = crate::Reg<wlan_rx_abort::WlanRxAbortSpec>;
#[doc = "WLAN receive abort configuration"]
pub mod wlan_rx_abort;
#[doc = "config_2 (rw) register accessor: Arbitration configuration register 2 (PTA_REG_CONFIG2)\n\nYou can [`read`](crate::Reg::read) this register and get [`config_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config_2`] module"]
#[doc(alias = "config_2")]
pub type Config2 = crate::Reg<config_2::Config2Spec>;
#[doc = "Arbitration configuration register 2 (PTA_REG_CONFIG2)"]
pub mod config_2;
