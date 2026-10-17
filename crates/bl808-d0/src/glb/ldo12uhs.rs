#[doc = "Register `LDO12UHS` reader"]
pub type R = crate::R<Ldo12uhsSpec>;
#[doc = "Register `LDO12UHS` writer"]
pub type W = crate::W<Ldo12uhsSpec>;
#[doc = "Field `pu_ldo12uhs` reader - "]
pub type PuLdo12uhsR = crate::BitReader;
#[doc = "Field `pu_ldo12uhs` writer - "]
pub type PuLdo12uhsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo12uhs_bypass` reader - "]
pub type Ldo12uhsBypassR = crate::BitReader;
#[doc = "Field `ldo12uhs_bypass` writer - "]
pub type Ldo12uhsBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo12uhs_pulldown` reader - "]
pub type Ldo12uhsPulldownR = crate::BitReader;
#[doc = "Field `ldo12uhs_pulldown` writer - "]
pub type Ldo12uhsPulldownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo12uhs_pulldown_sel` reader - "]
pub type Ldo12uhsPulldownSelR = crate::BitReader;
#[doc = "Field `ldo12uhs_pulldown_sel` writer - "]
pub type Ldo12uhsPulldownSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo12uhs_bm` reader - "]
pub type Ldo12uhsBmR = crate::FieldReader;
#[doc = "Field `ldo12uhs_bm` writer - "]
pub type Ldo12uhsBmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo12uhs_cc` reader - "]
pub type Ldo12uhsCcR = crate::FieldReader;
#[doc = "Field `ldo12uhs_cc` writer - "]
pub type Ldo12uhsCcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo12uhs_ocp_out` reader - "]
pub type Ldo12uhsOcpOutR = crate::BitReader;
#[doc = "Field `ldo12uhs_ocp_th` reader - "]
pub type Ldo12uhsOcpThR = crate::FieldReader;
#[doc = "Field `ldo12uhs_ocp_th` writer - "]
pub type Ldo12uhsOcpThW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo12uhs_ocp_en` reader - "]
pub type Ldo12uhsOcpEnR = crate::BitReader;
#[doc = "Field `ldo12uhs_ocp_en` writer - "]
pub type Ldo12uhsOcpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo12uhs_sstart_delay` reader - "]
pub type Ldo12uhsSstartDelayR = crate::FieldReader;
#[doc = "Field `ldo12uhs_sstart_delay` writer - "]
pub type Ldo12uhsSstartDelayW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo12uhs_sstart_en` reader - "]
pub type Ldo12uhsSstartEnR = crate::BitReader;
#[doc = "Field `ldo12uhs_sstart_en` writer - "]
pub type Ldo12uhsSstartEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo12uhs_vout_sel` reader - "]
pub type Ldo12uhsVoutSelR = crate::FieldReader;
#[doc = "Field `ldo12uhs_vout_sel` writer - "]
pub type Ldo12uhsVoutSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ldo12uhs_vout_trim` reader - "]
pub type Ldo12uhsVoutTrimR = crate::FieldReader;
#[doc = "Field `ldo12uhs_vout_trim` writer - "]
pub type Ldo12uhsVoutTrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ldo12uhs(&self) -> PuLdo12uhsR {
        PuLdo12uhsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ldo12uhs_bypass(&self) -> Ldo12uhsBypassR {
        Ldo12uhsBypassR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ldo12uhs_pulldown(&self) -> Ldo12uhsPulldownR {
        Ldo12uhsPulldownR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ldo12uhs_pulldown_sel(&self) -> Ldo12uhsPulldownSelR {
        Ldo12uhsPulldownSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn ldo12uhs_bm(&self) -> Ldo12uhsBmR {
        Ldo12uhsBmR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn ldo12uhs_cc(&self) -> Ldo12uhsCcR {
        Ldo12uhsCcR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ldo12uhs_ocp_out(&self) -> Ldo12uhsOcpOutR {
        Ldo12uhsOcpOutR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn ldo12uhs_ocp_th(&self) -> Ldo12uhsOcpThR {
        Ldo12uhsOcpThR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ldo12uhs_ocp_en(&self) -> Ldo12uhsOcpEnR {
        Ldo12uhsOcpEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn ldo12uhs_sstart_delay(&self) -> Ldo12uhsSstartDelayR {
        Ldo12uhsSstartDelayR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ldo12uhs_sstart_en(&self) -> Ldo12uhsSstartEnR {
        Ldo12uhsSstartEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn ldo12uhs_vout_sel(&self) -> Ldo12uhsVoutSelR {
        Ldo12uhsVoutSelR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn ldo12uhs_vout_trim(&self) -> Ldo12uhsVoutTrimR {
        Ldo12uhsVoutTrimR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ldo12uhs(&mut self) -> PuLdo12uhsW<'_, Ldo12uhsSpec> {
        PuLdo12uhsW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ldo12uhs_bypass(&mut self) -> Ldo12uhsBypassW<'_, Ldo12uhsSpec> {
        Ldo12uhsBypassW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ldo12uhs_pulldown(&mut self) -> Ldo12uhsPulldownW<'_, Ldo12uhsSpec> {
        Ldo12uhsPulldownW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ldo12uhs_pulldown_sel(&mut self) -> Ldo12uhsPulldownSelW<'_, Ldo12uhsSpec> {
        Ldo12uhsPulldownSelW::new(self, 3)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn ldo12uhs_bm(&mut self) -> Ldo12uhsBmW<'_, Ldo12uhsSpec> {
        Ldo12uhsBmW::new(self, 4)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn ldo12uhs_cc(&mut self) -> Ldo12uhsCcW<'_, Ldo12uhsSpec> {
        Ldo12uhsCcW::new(self, 8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn ldo12uhs_ocp_th(&mut self) -> Ldo12uhsOcpThW<'_, Ldo12uhsSpec> {
        Ldo12uhsOcpThW::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ldo12uhs_ocp_en(&mut self) -> Ldo12uhsOcpEnW<'_, Ldo12uhsSpec> {
        Ldo12uhsOcpEnW::new(self, 15)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn ldo12uhs_sstart_delay(&mut self) -> Ldo12uhsSstartDelayW<'_, Ldo12uhsSpec> {
        Ldo12uhsSstartDelayW::new(self, 16)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ldo12uhs_sstart_en(&mut self) -> Ldo12uhsSstartEnW<'_, Ldo12uhsSpec> {
        Ldo12uhsSstartEnW::new(self, 19)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn ldo12uhs_vout_sel(&mut self) -> Ldo12uhsVoutSelW<'_, Ldo12uhsSpec> {
        Ldo12uhsVoutSelW::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn ldo12uhs_vout_trim(&mut self) -> Ldo12uhsVoutTrimW<'_, Ldo12uhsSpec> {
        Ldo12uhsVoutTrimW::new(self, 24)
    }
}
#[doc = "LDO 1.2V UHS Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ldo12uhs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo12uhs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ldo12uhsSpec;
impl crate::RegisterSpec for Ldo12uhsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldo12uhs::R`](R) reader structure"]
impl crate::Readable for Ldo12uhsSpec {}
#[doc = "`write(|w| ..)` method takes [`ldo12uhs::W`](W) writer structure"]
impl crate::Writable for Ldo12uhsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LDO12UHS to value 0"]
impl crate::Resettable for Ldo12uhsSpec {}
