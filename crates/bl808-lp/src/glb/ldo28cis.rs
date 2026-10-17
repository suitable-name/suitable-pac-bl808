#[doc = "Register `LDO28CIS` reader"]
pub type R = crate::R<Ldo28cisSpec>;
#[doc = "Register `LDO28CIS` writer"]
pub type W = crate::W<Ldo28cisSpec>;
#[doc = "Field `pu_ldo28cis` reader - "]
pub type PuLdo28cisR = crate::BitReader;
#[doc = "Field `pu_ldo28cis` writer - "]
pub type PuLdo28cisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo28cis_bypass` reader - "]
pub type Ldo28cisBypassR = crate::BitReader;
#[doc = "Field `ldo28cis_bypass` writer - "]
pub type Ldo28cisBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo28cis_pulldown` reader - "]
pub type Ldo28cisPulldownR = crate::BitReader;
#[doc = "Field `ldo28cis_pulldown` writer - "]
pub type Ldo28cisPulldownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo28cis_pulldown_sel` reader - "]
pub type Ldo28cisPulldownSelR = crate::BitReader;
#[doc = "Field `ldo28cis_pulldown_sel` writer - "]
pub type Ldo28cisPulldownSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo28cis_bm` reader - "]
pub type Ldo28cisBmR = crate::FieldReader;
#[doc = "Field `ldo28cis_bm` writer - "]
pub type Ldo28cisBmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo28cis_cc` reader - "]
pub type Ldo28cisCcR = crate::FieldReader;
#[doc = "Field `ldo28cis_cc` writer - "]
pub type Ldo28cisCcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo28cis_ocp_out` reader - "]
pub type Ldo28cisOcpOutR = crate::BitReader;
#[doc = "Field `ldo28cis_ocp_th` reader - "]
pub type Ldo28cisOcpThR = crate::FieldReader;
#[doc = "Field `ldo28cis_ocp_th` writer - "]
pub type Ldo28cisOcpThW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo28cis_ocp_en` reader - "]
pub type Ldo28cisOcpEnR = crate::BitReader;
#[doc = "Field `ldo28cis_ocp_en` writer - "]
pub type Ldo28cisOcpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo28cis_sstart_delay` reader - "]
pub type Ldo28cisSstartDelayR = crate::FieldReader;
#[doc = "Field `ldo28cis_sstart_delay` writer - "]
pub type Ldo28cisSstartDelayW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo28cis_sstart_en` reader - "]
pub type Ldo28cisSstartEnR = crate::BitReader;
#[doc = "Field `ldo28cis_sstart_en` writer - "]
pub type Ldo28cisSstartEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo28cis_vout_sel` reader - "]
pub type Ldo28cisVoutSelR = crate::FieldReader;
#[doc = "Field `ldo28cis_vout_sel` writer - "]
pub type Ldo28cisVoutSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ldo28cis_vout_trim` reader - "]
pub type Ldo28cisVoutTrimR = crate::FieldReader;
#[doc = "Field `ldo28cis_vout_trim` writer - "]
pub type Ldo28cisVoutTrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ldo28cis(&self) -> PuLdo28cisR {
        PuLdo28cisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ldo28cis_bypass(&self) -> Ldo28cisBypassR {
        Ldo28cisBypassR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ldo28cis_pulldown(&self) -> Ldo28cisPulldownR {
        Ldo28cisPulldownR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ldo28cis_pulldown_sel(&self) -> Ldo28cisPulldownSelR {
        Ldo28cisPulldownSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn ldo28cis_bm(&self) -> Ldo28cisBmR {
        Ldo28cisBmR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn ldo28cis_cc(&self) -> Ldo28cisCcR {
        Ldo28cisCcR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ldo28cis_ocp_out(&self) -> Ldo28cisOcpOutR {
        Ldo28cisOcpOutR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn ldo28cis_ocp_th(&self) -> Ldo28cisOcpThR {
        Ldo28cisOcpThR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ldo28cis_ocp_en(&self) -> Ldo28cisOcpEnR {
        Ldo28cisOcpEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn ldo28cis_sstart_delay(&self) -> Ldo28cisSstartDelayR {
        Ldo28cisSstartDelayR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ldo28cis_sstart_en(&self) -> Ldo28cisSstartEnR {
        Ldo28cisSstartEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn ldo28cis_vout_sel(&self) -> Ldo28cisVoutSelR {
        Ldo28cisVoutSelR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn ldo28cis_vout_trim(&self) -> Ldo28cisVoutTrimR {
        Ldo28cisVoutTrimR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ldo28cis(&mut self) -> PuLdo28cisW<'_, Ldo28cisSpec> {
        PuLdo28cisW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ldo28cis_bypass(&mut self) -> Ldo28cisBypassW<'_, Ldo28cisSpec> {
        Ldo28cisBypassW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ldo28cis_pulldown(&mut self) -> Ldo28cisPulldownW<'_, Ldo28cisSpec> {
        Ldo28cisPulldownW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ldo28cis_pulldown_sel(&mut self) -> Ldo28cisPulldownSelW<'_, Ldo28cisSpec> {
        Ldo28cisPulldownSelW::new(self, 3)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn ldo28cis_bm(&mut self) -> Ldo28cisBmW<'_, Ldo28cisSpec> {
        Ldo28cisBmW::new(self, 4)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn ldo28cis_cc(&mut self) -> Ldo28cisCcW<'_, Ldo28cisSpec> {
        Ldo28cisCcW::new(self, 8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn ldo28cis_ocp_th(&mut self) -> Ldo28cisOcpThW<'_, Ldo28cisSpec> {
        Ldo28cisOcpThW::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ldo28cis_ocp_en(&mut self) -> Ldo28cisOcpEnW<'_, Ldo28cisSpec> {
        Ldo28cisOcpEnW::new(self, 15)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn ldo28cis_sstart_delay(&mut self) -> Ldo28cisSstartDelayW<'_, Ldo28cisSpec> {
        Ldo28cisSstartDelayW::new(self, 16)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ldo28cis_sstart_en(&mut self) -> Ldo28cisSstartEnW<'_, Ldo28cisSpec> {
        Ldo28cisSstartEnW::new(self, 19)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn ldo28cis_vout_sel(&mut self) -> Ldo28cisVoutSelW<'_, Ldo28cisSpec> {
        Ldo28cisVoutSelW::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn ldo28cis_vout_trim(&mut self) -> Ldo28cisVoutTrimW<'_, Ldo28cisSpec> {
        Ldo28cisVoutTrimW::new(self, 24)
    }
}
#[doc = "LDO 2.8V CIS Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ldo28cis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo28cis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ldo28cisSpec;
impl crate::RegisterSpec for Ldo28cisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldo28cis::R`](R) reader structure"]
impl crate::Readable for Ldo28cisSpec {}
#[doc = "`write(|w| ..)` method takes [`ldo28cis::W`](W) writer structure"]
impl crate::Writable for Ldo28cisSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LDO28CIS to value 0"]
impl crate::Resettable for Ldo28cisSpec {}
