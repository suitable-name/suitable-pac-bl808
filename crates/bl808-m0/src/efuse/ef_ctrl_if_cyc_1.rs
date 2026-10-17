#[doc = "Register `EF_CTRL_IF_CYC_1` reader"]
pub type R = crate::R<EfCtrlIfCyc1Spec>;
#[doc = "Register `EF_CTRL_IF_CYC_1` writer"]
pub type W = crate::W<EfCtrlIfCyc1Spec>;
#[doc = "Field `ef_if_cyc_pi` reader - Program initialization cycle count"]
pub type EfIfCycPiR = crate::FieldReader;
#[doc = "Field `ef_if_cyc_pi` writer - Program initialization cycle count"]
pub type EfIfCycPiW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ef_if_cyc_pp` reader - Program pulse width cycle count"]
pub type EfIfCycPpR = crate::FieldReader;
#[doc = "Field `ef_if_cyc_pp` writer - Program pulse width cycle count"]
pub type EfIfCycPpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ef_if_cyc_wr_adr` reader - Write address cycle count"]
pub type EfIfCycWrAdrR = crate::FieldReader;
#[doc = "Field `ef_if_cyc_wr_adr` writer - Write address cycle count"]
pub type EfIfCycWrAdrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ef_if_cyc_ps_cs` reader - Program setup to chip select cycle count"]
pub type EfIfCycPsCsR = crate::FieldReader;
#[doc = "Field `ef_if_cyc_ps_cs` writer - Program setup to chip select cycle count"]
pub type EfIfCycPsCsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ef_if_cyc_pd_cs_h` reader - Power down to chip select high cycle count"]
pub type EfIfCycPdCsHR = crate::FieldReader;
#[doc = "Field `ef_if_cyc_pd_cs_h` writer - Power down to chip select high cycle count"]
pub type EfIfCycPdCsHW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Program initialization cycle count"]
    #[inline(always)]
    pub fn ef_if_cyc_pi(&self) -> EfIfCycPiR {
        EfIfCycPiR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:13 - Program pulse width cycle count"]
    #[inline(always)]
    pub fn ef_if_cyc_pp(&self) -> EfIfCycPpR {
        EfIfCycPpR::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 14:19 - Write address cycle count"]
    #[inline(always)]
    pub fn ef_if_cyc_wr_adr(&self) -> EfIfCycWrAdrR {
        EfIfCycWrAdrR::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bits 20:25 - Program setup to chip select cycle count"]
    #[inline(always)]
    pub fn ef_if_cyc_ps_cs(&self) -> EfIfCycPsCsR {
        EfIfCycPsCsR::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 26:31 - Power down to chip select high cycle count"]
    #[inline(always)]
    pub fn ef_if_cyc_pd_cs_h(&self) -> EfIfCycPdCsHR {
        EfIfCycPdCsHR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Program initialization cycle count"]
    #[inline(always)]
    pub fn ef_if_cyc_pi(&mut self) -> EfIfCycPiW<'_, EfCtrlIfCyc1Spec> {
        EfIfCycPiW::new(self, 0)
    }
    #[doc = "Bits 6:13 - Program pulse width cycle count"]
    #[inline(always)]
    pub fn ef_if_cyc_pp(&mut self) -> EfIfCycPpW<'_, EfCtrlIfCyc1Spec> {
        EfIfCycPpW::new(self, 6)
    }
    #[doc = "Bits 14:19 - Write address cycle count"]
    #[inline(always)]
    pub fn ef_if_cyc_wr_adr(&mut self) -> EfIfCycWrAdrW<'_, EfCtrlIfCyc1Spec> {
        EfIfCycWrAdrW::new(self, 14)
    }
    #[doc = "Bits 20:25 - Program setup to chip select cycle count"]
    #[inline(always)]
    pub fn ef_if_cyc_ps_cs(&mut self) -> EfIfCycPsCsW<'_, EfCtrlIfCyc1Spec> {
        EfIfCycPsCsW::new(self, 20)
    }
    #[doc = "Bits 26:31 - Power down to chip select high cycle count"]
    #[inline(always)]
    pub fn ef_if_cyc_pd_cs_h(&mut self) -> EfIfCycPdCsHW<'_, EfCtrlIfCyc1Spec> {
        EfIfCycPdCsHW::new(self, 26)
    }
}
#[doc = "eFuse Interface Cycle Timing 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_cyc_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_cyc_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfCtrlIfCyc1Spec;
impl crate::RegisterSpec for EfCtrlIfCyc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_ctrl_if_cyc_1::R`](R) reader structure"]
impl crate::Readable for EfCtrlIfCyc1Spec {}
#[doc = "`write(|w| ..)` method takes [`ef_ctrl_if_cyc_1::W`](W) writer structure"]
impl crate::Writable for EfCtrlIfCyc1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_CTRL_IF_CYC_1 to value 0x0020_6609"]
impl crate::Resettable for EfCtrlIfCyc1Spec {
    const RESET_VALUE: u32 = 0x0020_6609;
}
