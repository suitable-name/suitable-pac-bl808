#[doc = "Register `LDO18FLASH` reader"]
pub type R = crate::R<Ldo18flashSpec>;
#[doc = "Register `LDO18FLASH` writer"]
pub type W = crate::W<Ldo18flashSpec>;
#[doc = "Field `pu_ldo18flash` reader - "]
pub type PuLdo18flashR = crate::BitReader;
#[doc = "Field `pu_ldo18flash` writer - "]
pub type PuLdo18flashW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo18flash_bypass` reader - "]
pub type Ldo18flashBypassR = crate::BitReader;
#[doc = "Field `ldo18flash_bypass` writer - "]
pub type Ldo18flashBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo18flash_pulldown` reader - "]
pub type Ldo18flashPulldownR = crate::BitReader;
#[doc = "Field `ldo18flash_pulldown` writer - "]
pub type Ldo18flashPulldownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo18flash_pulldown_sel` reader - "]
pub type Ldo18flashPulldownSelR = crate::BitReader;
#[doc = "Field `ldo18flash_pulldown_sel` writer - "]
pub type Ldo18flashPulldownSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo18flash_bm` reader - "]
pub type Ldo18flashBmR = crate::FieldReader;
#[doc = "Field `ldo18flash_bm` writer - "]
pub type Ldo18flashBmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo18flash_cc` reader - "]
pub type Ldo18flashCcR = crate::FieldReader;
#[doc = "Field `ldo18flash_cc` writer - "]
pub type Ldo18flashCcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo18flash_ocp_out` reader - "]
pub type Ldo18flashOcpOutR = crate::BitReader;
#[doc = "Field `ldo18flash_ocp_th` reader - "]
pub type Ldo18flashOcpThR = crate::FieldReader;
#[doc = "Field `ldo18flash_ocp_th` writer - "]
pub type Ldo18flashOcpThW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo18flash_ocp_en` reader - "]
pub type Ldo18flashOcpEnR = crate::BitReader;
#[doc = "Field `ldo18flash_ocp_en` writer - "]
pub type Ldo18flashOcpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo18flash_sstart_delay` reader - "]
pub type Ldo18flashSstartDelayR = crate::FieldReader;
#[doc = "Field `ldo18flash_sstart_delay` writer - "]
pub type Ldo18flashSstartDelayW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ldo18flash_sstart_en` reader - "]
pub type Ldo18flashSstartEnR = crate::BitReader;
#[doc = "Field `ldo18flash_sstart_en` writer - "]
pub type Ldo18flashSstartEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ldo18flash_vout_sel` reader - "]
pub type Ldo18flashVoutSelR = crate::FieldReader;
#[doc = "Field `ldo18flash_vout_sel` writer - "]
pub type Ldo18flashVoutSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ldo18flash_vout_trim` reader - "]
pub type Ldo18flashVoutTrimR = crate::FieldReader;
#[doc = "Field `ldo18flash_vout_trim` writer - "]
pub type Ldo18flashVoutTrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ldo18flash(&self) -> PuLdo18flashR {
        PuLdo18flashR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ldo18flash_bypass(&self) -> Ldo18flashBypassR {
        Ldo18flashBypassR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ldo18flash_pulldown(&self) -> Ldo18flashPulldownR {
        Ldo18flashPulldownR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ldo18flash_pulldown_sel(&self) -> Ldo18flashPulldownSelR {
        Ldo18flashPulldownSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn ldo18flash_bm(&self) -> Ldo18flashBmR {
        Ldo18flashBmR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn ldo18flash_cc(&self) -> Ldo18flashCcR {
        Ldo18flashCcR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ldo18flash_ocp_out(&self) -> Ldo18flashOcpOutR {
        Ldo18flashOcpOutR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn ldo18flash_ocp_th(&self) -> Ldo18flashOcpThR {
        Ldo18flashOcpThR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ldo18flash_ocp_en(&self) -> Ldo18flashOcpEnR {
        Ldo18flashOcpEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn ldo18flash_sstart_delay(&self) -> Ldo18flashSstartDelayR {
        Ldo18flashSstartDelayR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ldo18flash_sstart_en(&self) -> Ldo18flashSstartEnR {
        Ldo18flashSstartEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn ldo18flash_vout_sel(&self) -> Ldo18flashVoutSelR {
        Ldo18flashVoutSelR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn ldo18flash_vout_trim(&self) -> Ldo18flashVoutTrimR {
        Ldo18flashVoutTrimR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ldo18flash(&mut self) -> PuLdo18flashW<'_, Ldo18flashSpec> {
        PuLdo18flashW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ldo18flash_bypass(&mut self) -> Ldo18flashBypassW<'_, Ldo18flashSpec> {
        Ldo18flashBypassW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ldo18flash_pulldown(&mut self) -> Ldo18flashPulldownW<'_, Ldo18flashSpec> {
        Ldo18flashPulldownW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ldo18flash_pulldown_sel(&mut self) -> Ldo18flashPulldownSelW<'_, Ldo18flashSpec> {
        Ldo18flashPulldownSelW::new(self, 3)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn ldo18flash_bm(&mut self) -> Ldo18flashBmW<'_, Ldo18flashSpec> {
        Ldo18flashBmW::new(self, 4)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn ldo18flash_cc(&mut self) -> Ldo18flashCcW<'_, Ldo18flashSpec> {
        Ldo18flashCcW::new(self, 8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn ldo18flash_ocp_th(&mut self) -> Ldo18flashOcpThW<'_, Ldo18flashSpec> {
        Ldo18flashOcpThW::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ldo18flash_ocp_en(&mut self) -> Ldo18flashOcpEnW<'_, Ldo18flashSpec> {
        Ldo18flashOcpEnW::new(self, 15)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn ldo18flash_sstart_delay(&mut self) -> Ldo18flashSstartDelayW<'_, Ldo18flashSpec> {
        Ldo18flashSstartDelayW::new(self, 16)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ldo18flash_sstart_en(&mut self) -> Ldo18flashSstartEnW<'_, Ldo18flashSpec> {
        Ldo18flashSstartEnW::new(self, 19)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn ldo18flash_vout_sel(&mut self) -> Ldo18flashVoutSelW<'_, Ldo18flashSpec> {
        Ldo18flashVoutSelW::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn ldo18flash_vout_trim(&mut self) -> Ldo18flashVoutTrimW<'_, Ldo18flashSpec> {
        Ldo18flashVoutTrimW::new(self, 24)
    }
}
#[doc = "LDO 1.8V Flash Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ldo18flash::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldo18flash::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ldo18flashSpec;
impl crate::RegisterSpec for Ldo18flashSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldo18flash::R`](R) reader structure"]
impl crate::Readable for Ldo18flashSpec {}
#[doc = "`write(|w| ..)` method takes [`ldo18flash::W`](W) writer structure"]
impl crate::Writable for Ldo18flashSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LDO18FLASH to value 0"]
impl crate::Resettable for Ldo18flashSpec {}
