#[doc = "Register `IRRX_CONFIG` reader"]
pub type R = crate::R<IrrxConfigSpec>;
#[doc = "Register `IRRX_CONFIG` writer"]
pub type W = crate::W<IrrxConfigSpec>;
#[doc = "Field `cr_irrx_en` reader - Enable IRRX module"]
pub type CrIrrxEnR = crate::BitReader;
#[doc = "Field `cr_irrx_en` writer - Enable IRRX module"]
pub type CrIrrxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irrx_in_inv` reader - Invert IRRX input signal"]
pub type CrIrrxInInvR = crate::BitReader;
#[doc = "Field `cr_irrx_in_inv` writer - Invert IRRX input signal"]
pub type CrIrrxInInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irrx_mode` reader - IRRX receiving mode"]
pub type CrIrrxModeR = crate::FieldReader;
#[doc = "Field `cr_irrx_mode` writer - IRRX receiving mode"]
pub type CrIrrxModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `cr_irrx_deg_en` reader - Enable deglitch function for IRRX"]
pub type CrIrrxDegEnR = crate::BitReader;
#[doc = "Field `cr_irrx_deg_en` writer - Enable deglitch function for IRRX"]
pub type CrIrrxDegEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cr_irrx_deg_cnt` reader - Deglitch cycle count"]
pub type CrIrrxDegCntR = crate::FieldReader;
#[doc = "Field `cr_irrx_deg_cnt` writer - Deglitch cycle count"]
pub type CrIrrxDegCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Enable IRRX module"]
    #[inline(always)]
    pub fn cr_irrx_en(&self) -> CrIrrxEnR {
        CrIrrxEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Invert IRRX input signal"]
    #[inline(always)]
    pub fn cr_irrx_in_inv(&self) -> CrIrrxInInvR {
        CrIrrxInInvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - IRRX receiving mode"]
    #[inline(always)]
    pub fn cr_irrx_mode(&self) -> CrIrrxModeR {
        CrIrrxModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Enable deglitch function for IRRX"]
    #[inline(always)]
    pub fn cr_irrx_deg_en(&self) -> CrIrrxDegEnR {
        CrIrrxDegEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Deglitch cycle count"]
    #[inline(always)]
    pub fn cr_irrx_deg_cnt(&self) -> CrIrrxDegCntR {
        CrIrrxDegCntR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable IRRX module"]
    #[inline(always)]
    pub fn cr_irrx_en(&mut self) -> CrIrrxEnW<'_, IrrxConfigSpec> {
        CrIrrxEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Invert IRRX input signal"]
    #[inline(always)]
    pub fn cr_irrx_in_inv(&mut self) -> CrIrrxInInvW<'_, IrrxConfigSpec> {
        CrIrrxInInvW::new(self, 1)
    }
    #[doc = "Bits 2:3 - IRRX receiving mode"]
    #[inline(always)]
    pub fn cr_irrx_mode(&mut self) -> CrIrrxModeW<'_, IrrxConfigSpec> {
        CrIrrxModeW::new(self, 2)
    }
    #[doc = "Bit 4 - Enable deglitch function for IRRX"]
    #[inline(always)]
    pub fn cr_irrx_deg_en(&mut self) -> CrIrrxDegEnW<'_, IrrxConfigSpec> {
        CrIrrxDegEnW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Deglitch cycle count"]
    #[inline(always)]
    pub fn cr_irrx_deg_cnt(&mut self) -> CrIrrxDegCntW<'_, IrrxConfigSpec> {
        CrIrrxDegCntW::new(self, 8)
    }
}
#[doc = "IRRX Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irrx_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrrxConfigSpec;
impl crate::RegisterSpec for IrrxConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irrx_config::R`](R) reader structure"]
impl crate::Readable for IrrxConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`irrx_config::W`](W) writer structure"]
impl crate::Writable for IrrxConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRRX_CONFIG to value 0x02"]
impl crate::Resettable for IrrxConfigSpec {
    const RESET_VALUE: u32 = 0x02;
}
