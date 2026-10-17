#[doc = "Register `I2S_BCLK_CONFIG` reader"]
pub type R = crate::R<I2sBclkConfigSpec>;
#[doc = "Register `I2S_BCLK_CONFIG` writer"]
pub type W = crate::W<I2sBclkConfigSpec>;
#[doc = "Field `cr_bclk_div_l` reader - BCLK low period cycles"]
pub type CrBclkDivLR = crate::FieldReader<u16>;
#[doc = "Field `cr_bclk_div_l` writer - BCLK low period cycles"]
pub type CrBclkDivLW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `cr_bclk_div_h` reader - BCLK high period cycles"]
pub type CrBclkDivHR = crate::FieldReader<u16>;
#[doc = "Field `cr_bclk_div_h` writer - BCLK high period cycles"]
pub type CrBclkDivHW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - BCLK low period cycles"]
    #[inline(always)]
    pub fn cr_bclk_div_l(&self) -> CrBclkDivLR {
        CrBclkDivLR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - BCLK high period cycles"]
    #[inline(always)]
    pub fn cr_bclk_div_h(&self) -> CrBclkDivHR {
        CrBclkDivHR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - BCLK low period cycles"]
    #[inline(always)]
    pub fn cr_bclk_div_l(&mut self) -> CrBclkDivLW<'_, I2sBclkConfigSpec> {
        CrBclkDivLW::new(self, 0)
    }
    #[doc = "Bits 16:27 - BCLK high period cycles"]
    #[inline(always)]
    pub fn cr_bclk_div_h(&mut self) -> CrBclkDivHW<'_, I2sBclkConfigSpec> {
        CrBclkDivHW::new(self, 16)
    }
}
#[doc = "BCLK Divider Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s_bclk_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s_bclk_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sBclkConfigSpec;
impl crate::RegisterSpec for I2sBclkConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_bclk_config::R`](R) reader structure"]
impl crate::Readable for I2sBclkConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`i2s_bclk_config::W`](W) writer structure"]
impl crate::Writable for I2sBclkConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2S_BCLK_CONFIG to value 0x0001_0001"]
impl crate::Resettable for I2sBclkConfigSpec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
