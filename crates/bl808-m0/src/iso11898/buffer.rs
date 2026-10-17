#[repr(C)]
#[doc = "Multiplexed Data/Filter Buffer Cluster"]
#[doc(alias = "BUFFER")]
pub struct Buffer {
    reg: Reg,
}
impl Buffer {
    #[doc = "0x00 - CAN Data/Filter Access Register. Maps to RX/TX data in Operating Mode or Acceptance Filter parameters in Reset Mode."]
    #[inline(always)]
    pub const fn reg(&self) -> &Reg {
        &self.reg
    }
}
#[doc = "REG (rw) register accessor: CAN Data/Filter Access Register. Maps to RX/TX data in Operating Mode or Acceptance Filter parameters in Reset Mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg`] module"]
#[doc(alias = "REG")]
pub type Reg = crate::Reg<reg::RegSpec>;
#[doc = "CAN Data/Filter Access Register. Maps to RX/TX data in Operating Mode or Acceptance Filter parameters in Reset Mode."]
pub mod reg;
