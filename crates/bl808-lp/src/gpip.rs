#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gpadc_config: GpadcConfig,
    gpadc_dma_rdata: GpadcDmaRdata,
    _reserved2: [u8; 0x18],
    gpadc_pir_train: GpadcPirTrain,
    _reserved3: [u8; 0x1c],
    gpdac_config: GpdacConfig,
    gpdac_dma_config: GpdacDmaConfig,
    gpdac_dma_wdata: GpdacDmaWdata,
    gpdac_tx_fifo_status: GpdacTxFifoStatus,
}
impl RegisterBlock {
    #[doc = "0x00 - GPADC Configuration and Interrupt Register"]
    #[inline(always)]
    pub const fn gpadc_config(&self) -> &GpadcConfig {
        &self.gpadc_config
    }
    #[doc = "0x04 - GPADC DMA Read Data Register"]
    #[inline(always)]
    pub const fn gpadc_dma_rdata(&self) -> &GpadcDmaRdata {
        &self.gpadc_dma_rdata
    }
    #[doc = "0x20 - GPADC PIR Training Register"]
    #[inline(always)]
    pub const fn gpadc_pir_train(&self) -> &GpadcPirTrain {
        &self.gpadc_pir_train
    }
    #[doc = "0x40 - GPDAC Configuration Register"]
    #[inline(always)]
    pub const fn gpdac_config(&self) -> &GpdacConfig {
        &self.gpdac_config
    }
    #[doc = "0x44 - GPDAC DMA Configuration Register"]
    #[inline(always)]
    pub const fn gpdac_dma_config(&self) -> &GpdacDmaConfig {
        &self.gpdac_dma_config
    }
    #[doc = "0x48 - GPDAC DMA Write Data Register"]
    #[inline(always)]
    pub const fn gpdac_dma_wdata(&self) -> &GpdacDmaWdata {
        &self.gpdac_dma_wdata
    }
    #[doc = "0x4c - GPDAC TX FIFO Status Register"]
    #[inline(always)]
    pub const fn gpdac_tx_fifo_status(&self) -> &GpdacTxFifoStatus {
        &self.gpdac_tx_fifo_status
    }
}
#[doc = "GPADC_CONFIG (rw) register accessor: GPADC Configuration and Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_config`] module"]
#[doc(alias = "GPADC_CONFIG")]
pub type GpadcConfig = crate::Reg<gpadc_config::GpadcConfigSpec>;
#[doc = "GPADC Configuration and Interrupt Register"]
pub mod gpadc_config;
#[doc = "GPADC_DMA_RDATA (r) register accessor: GPADC DMA Read Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_dma_rdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_dma_rdata`] module"]
#[doc(alias = "GPADC_DMA_RDATA")]
pub type GpadcDmaRdata = crate::Reg<gpadc_dma_rdata::GpadcDmaRdataSpec>;
#[doc = "GPADC DMA Read Data Register"]
pub mod gpadc_dma_rdata;
#[doc = "GPADC_PIR_TRAIN (rw) register accessor: GPADC PIR Training Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpadc_pir_train::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpadc_pir_train::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpadc_pir_train`] module"]
#[doc(alias = "GPADC_PIR_TRAIN")]
pub type GpadcPirTrain = crate::Reg<gpadc_pir_train::GpadcPirTrainSpec>;
#[doc = "GPADC PIR Training Register"]
pub mod gpadc_pir_train;
#[doc = "GPDAC_CONFIG (rw) register accessor: GPDAC Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpdac_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdac_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpdac_config`] module"]
#[doc(alias = "GPDAC_CONFIG")]
pub type GpdacConfig = crate::Reg<gpdac_config::GpdacConfigSpec>;
#[doc = "GPDAC Configuration Register"]
pub mod gpdac_config;
#[doc = "GPDAC_DMA_CONFIG (rw) register accessor: GPDAC DMA Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpdac_dma_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdac_dma_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpdac_dma_config`] module"]
#[doc(alias = "GPDAC_DMA_CONFIG")]
pub type GpdacDmaConfig = crate::Reg<gpdac_dma_config::GpdacDmaConfigSpec>;
#[doc = "GPDAC DMA Configuration Register"]
pub mod gpdac_dma_config;
#[doc = "GPDAC_DMA_WDATA (w) register accessor: GPDAC DMA Write Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpdac_dma_wdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpdac_dma_wdata`] module"]
#[doc(alias = "GPDAC_DMA_WDATA")]
pub type GpdacDmaWdata = crate::Reg<gpdac_dma_wdata::GpdacDmaWdataSpec>;
#[doc = "GPDAC DMA Write Data Register"]
pub mod gpdac_dma_wdata;
#[doc = "GPDAC_TX_FIFO_STATUS (r) register accessor: GPDAC TX FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpdac_tx_fifo_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpdac_tx_fifo_status`] module"]
#[doc(alias = "GPDAC_TX_FIFO_STATUS")]
pub type GpdacTxFifoStatus = crate::Reg<gpdac_tx_fifo_status::GpdacTxFifoStatusSpec>;
#[doc = "GPDAC TX FIFO Status Register"]
pub mod gpdac_tx_fifo_status;
