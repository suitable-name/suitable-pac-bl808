#[doc = "Register `LDO15CIS` reader"]
pub type R = crate::R<Ldo15cisSpec>;
#[doc = "Register `LDO15CIS` writer"]
pub type W = crate::W<Ldo15cisSpec>;
#[doc = "Field `pu_ldo15cis` reader - "]
pub type PuLdo15cisR = crate::BitReader;
#[doc = "Field `pu_ldo15cis` writer - "]
pub type PuLdo15cisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo15cis_bypass` reader - "]
pub type Ldo15cisBypassR = crate::BitReader;
#[doc = "Field `ldo15cis_bypass` writer - "]
pub type Ldo15cisBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo15cis_pulldown` reader - "]
pub type Ldo15cisPulldownR = crate::BitReader;
#[doc = "Field `ldo15cis_pulldown` writer - "]
pub type Ldo15cisPulldownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo15cis_pulldown_sel` reader - "]
pub type Ldo15cisPulldownSelR = crate::BitReader;
#[doc = "Field `ldo15cis_pulldown_sel` writer - "]
pub type Ldo15cisPulldownSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo15cis_bm` reader - "]
pub type Ldo15cisBmR = crate::FieldReader;
#[doc = "Field `ldo15cis_bm` writer - "]
pub type Ldo15cisBmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo15cis_cc` reader - "]
pub type Ldo15cisCcR = crate::FieldReader;
#[doc = "Field `ldo15cis_cc` writer - "]
pub type Ldo15cisCcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo15cis_ocp_out` reader - "]
pub type Ldo15cisOcpOutR = crate::BitReader;
#[doc = "Field `ldo15cis_ocp_th` reader - "]
pub type Ldo15cisOcpThR = crate::FieldReader;
#[doc = "Field `ldo15cis_ocp_th` writer - "]
pub type Ldo15cisOcpThW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo15cis_ocp_en` reader - "]
pub type Ldo15cisOcpEnR = crate::BitReader;
#[doc = "Field `ldo15cis_ocp_en` writer - "]
pub type Ldo15cisOcpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo15cis_sstart_delay` reader - "]
pub type Ldo15cisSstartDelayR = crate::FieldReader;
#[doc = "Field `ldo15cis_sstart_delay` writer - "]
pub type Ldo15cisSstartDelayW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo15cis_sstart_en` reader - "]
pub type Ldo15cisSstartEnR = crate::BitReader;
#[doc = "Field `ldo15cis_sstart_en` writer - "]
pub type Ldo15cisSstartEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo15cis_vout_sel` reader - "]
pub type Ldo15cisVoutSelR = crate::FieldReader;
#[doc = "Field `ldo15cis_vout_sel` writer - "]
pub type Ldo15cisVoutSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ldo15cis_vout_trim` reader - "]
pub type Ldo15cisVoutTrimR = crate::FieldReader;
#[doc = "Field `ldo15cis_vout_trim` writer - "]
pub type Ldo15cisVoutTrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ldo15cis(&self) -> PuLdo15cisR {
        PuLdo15cisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ldo15cis_bypass(&self) -> Ldo15cisBypassR {
        Ldo15cisBypassR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ldo15cis_pulldown(&self) -> Ldo15cisPulldownR {
        Ldo15cisPulldownR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ldo15cis_pulldown_sel(&self) -> Ldo15cisPulldownSelR {
        Ldo15cisPulldownSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn ldo15cis_bm(&self) -> Ldo15cisBmR {
        Ldo15cisBmR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn ldo15cis_cc(&self) -> Ldo15cisCcR {
        Ldo15cisCcR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ldo15cis_ocp_out(&self) -> Ldo15cisOcpOutR {
        Ldo15cisOcpOutR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn ldo15cis_ocp_th(&self) -> Ldo15cisOcpThR {
        Ldo15cisOcpThR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ldo15cis_ocp_en(&self) -> Ldo15cisOcpEnR {
        Ldo15cisOcpEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn ldo15cis_sstart_delay(&self) -> Ldo15cisSstartDelayR {
        Ldo15cisSstartDelayR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ldo15cis_sstart_en(&self) -> Ldo15cisSstartEnR {
        Ldo15cisSstartEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn ldo15cis_vout_sel(&self) -> Ldo15cisVoutSelR {
        Ldo15cisVoutSelR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn ldo15cis_vout_trim(&self) -> Ldo15cisVoutTrimR {
        Ldo15cisVoutTrimR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ldo15cis(&mut self) -> PuLdo15cisW<'_, Ldo15cisSpec> {
        PuLdo15cisW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ldo15cis_bypass(&mut self) -> Ldo15cisBypassW<'_, Ldo15cisSpec> {
        Ldo15cisBypassW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ldo15cis_pulldown(&mut self) -> Ldo15cisPulldownW<'_, Ldo15cisSpec> {
        Ldo15cisPulldownW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ldo15cis_pulldown_sel(&mut self) -> Ldo15cisPulldownSelW<'_, Ldo15cisSpec> {
        Ldo15cisPulldownSelW::new(self, 3)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn ldo15cis_bm(&mut self) -> Ldo15cisBmW<'_, Ldo15cisSpec> {
        Ldo15cisBmW::new(self, 4)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn ldo15cis_cc(&mut self) -> Ldo15cisCcW<'_, Ldo15cisSpec> {
        Ldo15cisCcW::new(self, 8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn ldo15cis_ocp_th(&mut self) -> Ldo15cisOcpThW<'_, Ldo15cisSpec> {
        Ldo15cisOcpThW::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ldo15cis_ocp_en(&mut self) -> Ldo15cisOcpEnW<'_, Ldo15cisSpec> {
        Ldo15cisOcpEnW::new(self, 15)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn ldo15cis_sstart_delay(&mut self) -> Ldo15cisSstartDelayW<'_, Ldo15cisSpec> {
        Ldo15cisSstartDelayW::new(self, 16)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ldo15cis_sstart_en(&mut self) -> Ldo15cisSstartEnW<'_, Ldo15cisSpec> {
        Ldo15cisSstartEnW::new(self, 19)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn ldo15cis_vout_sel(&mut self) -> Ldo15cisVoutSelW<'_, Ldo15cisSpec> {
        Ldo15cisVoutSelW::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn ldo15cis_vout_trim(&mut self) -> Ldo15cisVoutTrimW<'_, Ldo15cisSpec> {
        Ldo15cisVoutTrimW::new(self, 24)
    }
}
#[doc = "LDO 1.5V CIS Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ldo15cis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo15cis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ldo15cisSpec;
impl crate::RegisterSpec for Ldo15cisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldo15cis::R`](R) reader structure"]
impl crate::Readable for Ldo15cisSpec {}
#[doc = "`write(|w| ..)` method takes [`ldo15cis::W`](W) writer structure"]
impl crate::Writable for Ldo15cisSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LDO15CIS to value 0"]
impl crate::Resettable for Ldo15cisSpec {}
