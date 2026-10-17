#[doc = "Register `WINBOND_PSRAM_CONFIGURE2` reader"]
pub type R = crate::R<WinbondPsramConfigure2Spec>;
#[doc = "Register `WINBOND_PSRAM_CONFIGURE2` writer"]
pub type W = crate::W<WinbondPsramConfigure2Spec>;
#[doc = "Field `reg_wb_zq_code` reader - Winbond ZQ calibration code"]
pub type RegWbZqCodeR = crate::FieldReader;
#[doc = "Field `reg_wb_zq_code` writer - Winbond ZQ calibration code"]
pub type RegWbZqCodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Winbond ZQ calibration code"]
    #[inline(always)]
    pub fn reg_wb_zq_code(&self) -> RegWbZqCodeR {
        RegWbZqCodeR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Winbond ZQ calibration code"]
    #[inline(always)]
    pub fn reg_wb_zq_code(&mut self) -> RegWbZqCodeW<'_, WinbondPsramConfigure2Spec> {
        RegWbZqCodeW::new(self, 0)
    }
}
#[doc = "Winbond Specific Configuration 2\n\nYou can [`read`](crate::Reg::read) this register and get [`winbond_psram_configure2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`winbond_psram_configure2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WinbondPsramConfigure2Spec;
impl crate::RegisterSpec for WinbondPsramConfigure2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`winbond_psram_configure2::R`](R) reader structure"]
impl crate::Readable for WinbondPsramConfigure2Spec {}
#[doc = "`write(|w| ..)` method takes [`winbond_psram_configure2::W`](W) writer structure"]
impl crate::Writable for WinbondPsramConfigure2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WINBOND_PSRAM_CONFIGURE2 to value 0"]
impl crate::Resettable for WinbondPsramConfigure2Spec {}
