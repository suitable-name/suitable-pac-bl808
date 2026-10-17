#[doc = "Register `EF_CTRL_IF_CYC_0` reader"]
pub type R = crate::R<EfCtrlIfCyc0Spec>;
#[doc = "Register `EF_CTRL_IF_CYC_0` writer"]
pub type W = crate::W<EfCtrlIfCyc0Spec>;
#[doc = "Field `ef_if_cyc_rd_dmy` reader - Read dummy cycle count"]
pub type EfIfCycRdDmyR = crate::FieldReader;
#[doc = "Field `ef_if_cyc_rd_dmy` writer - Read dummy cycle count"]
pub type EfIfCycRdDmyW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ef_if_cyc_rd_dat` reader - Read data cycle count"]
pub type EfIfCycRdDatR = crate::FieldReader;
#[doc = "Field `ef_if_cyc_rd_dat` writer - Read data cycle count"]
pub type EfIfCycRdDatW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ef_if_cyc_rd_adr` reader - Read address cycle count"]
pub type EfIfCycRdAdrR = crate::FieldReader;
#[doc = "Field `ef_if_cyc_rd_adr` writer - Read address cycle count"]
pub type EfIfCycRdAdrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ef_if_cyc_cs` reader - Chip select setup cycle count"]
pub type EfIfCycCsR = crate::FieldReader;
#[doc = "Field `ef_if_cyc_cs` writer - Chip select setup cycle count"]
pub type EfIfCycCsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ef_if_cyc_pd_cs_s` reader - Power down to chip select start cycle count"]
pub type EfIfCycPdCsSR = crate::FieldReader;
#[doc = "Field `ef_if_cyc_pd_cs_s` writer - Power down to chip select start cycle count"]
pub type EfIfCycPdCsSW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:5 - Read dummy cycle count"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_dmy(&self) -> EfIfCycRdDmyR {
        EfIfCycRdDmyR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Read data cycle count"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_dat(&self) -> EfIfCycRdDatR {
        EfIfCycRdDatR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - Read address cycle count"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_adr(&self) -> EfIfCycRdAdrR {
        EfIfCycRdAdrR::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - Chip select setup cycle count"]
    #[inline(always)]
    pub fn ef_if_cyc_cs(&self) -> EfIfCycCsR {
        EfIfCycCsR::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:31 - Power down to chip select start cycle count"]
    #[inline(always)]
    pub fn ef_if_cyc_pd_cs_s(&self) -> EfIfCycPdCsSR {
        EfIfCycPdCsSR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Read dummy cycle count"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_dmy(&mut self) -> EfIfCycRdDmyW<'_, EfCtrlIfCyc0Spec> {
        EfIfCycRdDmyW::new(self, 0)
    }
    #[doc = "Bits 6:11 - Read data cycle count"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_dat(&mut self) -> EfIfCycRdDatW<'_, EfCtrlIfCyc0Spec> {
        EfIfCycRdDatW::new(self, 6)
    }
    #[doc = "Bits 12:17 - Read address cycle count"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_adr(&mut self) -> EfIfCycRdAdrW<'_, EfCtrlIfCyc0Spec> {
        EfIfCycRdAdrW::new(self, 12)
    }
    #[doc = "Bits 18:23 - Chip select setup cycle count"]
    #[inline(always)]
    pub fn ef_if_cyc_cs(&mut self) -> EfIfCycCsW<'_, EfCtrlIfCyc0Spec> {
        EfIfCycCsW::new(self, 18)
    }
    #[doc = "Bits 24:31 - Power down to chip select start cycle count"]
    #[inline(always)]
    pub fn ef_if_cyc_pd_cs_s(&mut self) -> EfIfCycPdCsSW<'_, EfCtrlIfCyc0Spec> {
        EfIfCycPdCsSW::new(self, 24)
    }
}
#[doc = "eFuse Interface Cycle Timing 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_cyc_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_cyc_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfCtrlIfCyc0Spec;
impl crate::RegisterSpec for EfCtrlIfCyc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_ctrl_if_cyc_0::R`](R) reader structure"]
impl crate::Readable for EfCtrlIfCyc0Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_ctrl_if_cyc_0::W`](W) writer structure"]
impl crate::Writable for EfCtrlIfCyc0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_CTRL_IF_CYC_0 to value 0x1600_0040"]
impl crate::Resettable for EfCtrlIfCyc0Spec {
    const RESET_VALUE: u32 = 0x1600_0040;
}
