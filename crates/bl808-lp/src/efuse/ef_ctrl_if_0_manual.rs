#[doc = "Register `EF_CTRL_IF_0_MANUAL` reader"]
pub type R = crate::R<EfCtrlIf0ManualSpec>;
#[doc = "Register `EF_CTRL_IF_0_MANUAL` writer"]
pub type W = crate::W<EfCtrlIf0ManualSpec>;
#[doc = "Field `ef_if_a` reader - Manual access address"]
pub type EfIfAR = crate::FieldReader<u16>;
#[doc = "Field `ef_if_a` writer - Manual access address"]
pub type EfIfAW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `ef_if_pd` reader - Manual power down control"]
pub type EfIfPdR = crate::BitReader;
#[doc = "Field `ef_if_pd` writer - Manual power down control"]
pub type EfIfPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_ps` reader - Manual program voltage switch"]
pub type EfIfPsR = crate::BitReader;
#[doc = "Field `ef_if_ps` writer - Manual program voltage switch"]
pub type EfIfPsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_strobe` reader - Manual strobe signal"]
pub type EfIfStrobeR = crate::BitReader;
#[doc = "Field `ef_if_strobe` writer - Manual strobe signal"]
pub type EfIfStrobeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_pgenb` reader - Manual program enable bar"]
pub type EfIfPgenbR = crate::BitReader;
#[doc = "Field `ef_if_pgenb` writer - Manual program enable bar"]
pub type EfIfPgenbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_load` reader - Manual load signal"]
pub type EfIfLoadR = crate::BitReader;
#[doc = "Field `ef_if_load` writer - Manual load signal"]
pub type EfIfLoadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_csb` reader - Manual chip select bar"]
pub type EfIfCsbR = crate::BitReader;
#[doc = "Field `ef_if_csb` writer - Manual chip select bar"]
pub type EfIfCsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ef_if_0_q` reader - Manual read data output"]
pub type EfIf0QR = crate::FieldReader;
#[doc = "Field `ef_if_prot_code_manual` reader - Protection code for manual mode"]
pub type EfIfProtCodeManualR = crate::FieldReader;
#[doc = "Field `ef_if_prot_code_manual` writer - Protection code for manual mode"]
pub type EfIfProtCodeManualW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:9 - Manual access address"]
    #[inline(always)]
    pub fn ef_if_a(&self) -> EfIfAR {
        EfIfAR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Manual power down control"]
    #[inline(always)]
    pub fn ef_if_pd(&self) -> EfIfPdR {
        EfIfPdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Manual program voltage switch"]
    #[inline(always)]
    pub fn ef_if_ps(&self) -> EfIfPsR {
        EfIfPsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Manual strobe signal"]
    #[inline(always)]
    pub fn ef_if_strobe(&self) -> EfIfStrobeR {
        EfIfStrobeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Manual program enable bar"]
    #[inline(always)]
    pub fn ef_if_pgenb(&self) -> EfIfPgenbR {
        EfIfPgenbR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Manual load signal"]
    #[inline(always)]
    pub fn ef_if_load(&self) -> EfIfLoadR {
        EfIfLoadR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Manual chip select bar"]
    #[inline(always)]
    pub fn ef_if_csb(&self) -> EfIfCsbR {
        EfIfCsbR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Manual read data output"]
    #[inline(always)]
    pub fn ef_if_0_q(&self) -> EfIf0QR {
        EfIf0QR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Protection code for manual mode"]
    #[inline(always)]
    pub fn ef_if_prot_code_manual(&self) -> EfIfProtCodeManualR {
        EfIfProtCodeManualR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Manual access address"]
    #[inline(always)]
    pub fn ef_if_a(&mut self) -> EfIfAW<'_, EfCtrlIf0ManualSpec> {
        EfIfAW::new(self, 0)
    }
    #[doc = "Bit 10 - Manual power down control"]
    #[inline(always)]
    pub fn ef_if_pd(&mut self) -> EfIfPdW<'_, EfCtrlIf0ManualSpec> {
        EfIfPdW::new(self, 10)
    }
    #[doc = "Bit 11 - Manual program voltage switch"]
    #[inline(always)]
    pub fn ef_if_ps(&mut self) -> EfIfPsW<'_, EfCtrlIf0ManualSpec> {
        EfIfPsW::new(self, 11)
    }
    #[doc = "Bit 12 - Manual strobe signal"]
    #[inline(always)]
    pub fn ef_if_strobe(&mut self) -> EfIfStrobeW<'_, EfCtrlIf0ManualSpec> {
        EfIfStrobeW::new(self, 12)
    }
    #[doc = "Bit 13 - Manual program enable bar"]
    #[inline(always)]
    pub fn ef_if_pgenb(&mut self) -> EfIfPgenbW<'_, EfCtrlIf0ManualSpec> {
        EfIfPgenbW::new(self, 13)
    }
    #[doc = "Bit 14 - Manual load signal"]
    #[inline(always)]
    pub fn ef_if_load(&mut self) -> EfIfLoadW<'_, EfCtrlIf0ManualSpec> {
        EfIfLoadW::new(self, 14)
    }
    #[doc = "Bit 15 - Manual chip select bar"]
    #[inline(always)]
    pub fn ef_if_csb(&mut self) -> EfIfCsbW<'_, EfCtrlIf0ManualSpec> {
        EfIfCsbW::new(self, 15)
    }
    #[doc = "Bits 24:31 - Protection code for manual mode"]
    #[inline(always)]
    pub fn ef_if_prot_code_manual(&mut self) -> EfIfProtCodeManualW<'_, EfCtrlIf0ManualSpec> {
        EfIfProtCodeManualW::new(self, 24)
    }
}
#[doc = "eFuse Interface 0 Manual Operation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ef_ctrl_if_0_manual::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ef_ctrl_if_0_manual::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfCtrlIf0ManualSpec;
impl crate::RegisterSpec for EfCtrlIf0ManualSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ef_ctrl_if_0_manual::R`](R) reader structure"]
impl crate::Readable for EfCtrlIf0ManualSpec {}
#[doc = "`write(|w| ..)` method takes [`ef_ctrl_if_0_manual::W`](W) writer structure"]
impl crate::Writable for EfCtrlIf0ManualSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EF_CTRL_IF_0_MANUAL to value 0xe400"]
impl crate::Resettable for EfCtrlIf0ManualSpec {
    const RESET_VALUE: u32 = 0xe400;
}
