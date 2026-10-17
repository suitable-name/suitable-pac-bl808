#[doc = "Register `UHS_BASIC` reader"]
pub type R = crate::R<UhsBasicSpec>;
#[doc = "Register `UHS_BASIC` writer"]
pub type W = crate::W<UhsBasicSpec>;
#[doc = "Field `reg_init_en` reader - Initialization enable"]
pub type RegInitEnR = crate::BitReader;
#[doc = "Field `reg_init_en` writer - Initialization enable"]
pub type RegInitEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_af_en` reader - Auto-refresh enable"]
pub type RegAfEnR = crate::BitReader;
#[doc = "Field `reg_af_en` writer - Auto-refresh enable"]
pub type RegAfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_config_req` reader - Configuration request"]
pub type RegConfigReqR = crate::BitReader;
#[doc = "Field `reg_config_req` writer - Configuration request"]
pub type RegConfigReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_config_gnt` reader - Configuration grant status"]
pub type RegConfigGntR = crate::BitReader;
#[doc = "Field `reg_mode_reg` reader - UHS Mode Register configuration"]
pub type RegModeRegR = crate::FieldReader;
#[doc = "Field `reg_mode_reg` writer - UHS Mode Register configuration"]
pub type RegModeRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_addrmb_msk` reader - Address Mask for MB boundary"]
pub type RegAddrmbMskR = crate::FieldReader;
#[doc = "Field `reg_addrmb_msk` writer - Address Mask for MB boundary"]
pub type RegAddrmbMskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `reg_linear_bnd_b` reader - Linear burst boundary bit"]
pub type RegLinearBndBR = crate::FieldReader;
#[doc = "Field `reg_linear_bnd_b` writer - Linear burst boundary bit"]
pub type RegLinearBndBW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Initialization enable"]
    #[inline(always)]
    pub fn reg_init_en(&self) -> RegInitEnR {
        RegInitEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto-refresh enable"]
    #[inline(always)]
    pub fn reg_af_en(&self) -> RegAfEnR {
        RegAfEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configuration request"]
    #[inline(always)]
    pub fn reg_config_req(&self) -> RegConfigReqR {
        RegConfigReqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configuration grant status"]
    #[inline(always)]
    pub fn reg_config_gnt(&self) -> RegConfigGntR {
        RegConfigGntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:15 - UHS Mode Register configuration"]
    #[inline(always)]
    pub fn reg_mode_reg(&self) -> RegModeRegR {
        RegModeRegR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Address Mask for MB boundary"]
    #[inline(always)]
    pub fn reg_addrmb_msk(&self) -> RegAddrmbMskR {
        RegAddrmbMskR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 28:31 - Linear burst boundary bit"]
    #[inline(always)]
    pub fn reg_linear_bnd_b(&self) -> RegLinearBndBR {
        RegLinearBndBR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Initialization enable"]
    #[inline(always)]
    pub fn reg_init_en(&mut self) -> RegInitEnW<'_, UhsBasicSpec> {
        RegInitEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Auto-refresh enable"]
    #[inline(always)]
    pub fn reg_af_en(&mut self) -> RegAfEnW<'_, UhsBasicSpec> {
        RegAfEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Configuration request"]
    #[inline(always)]
    pub fn reg_config_req(&mut self) -> RegConfigReqW<'_, UhsBasicSpec> {
        RegConfigReqW::new(self, 2)
    }
    #[doc = "Bits 8:15 - UHS Mode Register configuration"]
    #[inline(always)]
    pub fn reg_mode_reg(&mut self) -> RegModeRegW<'_, UhsBasicSpec> {
        RegModeRegW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Address Mask for MB boundary"]
    #[inline(always)]
    pub fn reg_addrmb_msk(&mut self) -> RegAddrmbMskW<'_, UhsBasicSpec> {
        RegAddrmbMskW::new(self, 16)
    }
    #[doc = "Bits 28:31 - Linear burst boundary bit"]
    #[inline(always)]
    pub fn reg_linear_bnd_b(&mut self) -> RegLinearBndBW<'_, UhsBasicSpec> {
        RegLinearBndBW::new(self, 28)
    }
}
#[doc = "UHS Basic Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_basic::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_basic::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsBasicSpec;
impl crate::RegisterSpec for UhsBasicSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs_basic::R`](R) reader structure"]
impl crate::Readable for UhsBasicSpec {}
#[doc = "`write(|w| ..)` method takes [`uhs_basic::W`](W) writer structure"]
impl crate::Writable for UhsBasicSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHS_BASIC to value 0xa01f_0000"]
impl crate::Resettable for UhsBasicSpec {
    const RESET_VALUE: u32 = 0xa01f_0000;
}
