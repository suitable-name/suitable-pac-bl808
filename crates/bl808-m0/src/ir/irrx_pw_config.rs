#[doc = "Register `IRRX_PW_CONFIG` reader"]
pub type R = crate::R<IrrxPwConfigSpec>;
#[doc = "Register `IRRX_PW_CONFIG` writer"]
pub type W = crate::W<IrrxPwConfigSpec>;
#[doc = "Field `cr_irrx_data_th` reader - IRRX data threshold"]
pub type CrIrrxDataThR = crate::FieldReader<u16>;
#[doc = "Field `cr_irrx_data_th` writer - IRRX data threshold"]
pub type CrIrrxDataThW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `cr_irrx_end_th` reader - IRRX end-of-transfer threshold"]
pub type CrIrrxEndThR = crate::FieldReader<u16>;
#[doc = "Field `cr_irrx_end_th` writer - IRRX end-of-transfer threshold"]
pub type CrIrrxEndThW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IRRX data threshold"]
    #[inline(always)]
    pub fn cr_irrx_data_th(&self) -> CrIrrxDataThR {
        CrIrrxDataThR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - IRRX end-of-transfer threshold"]
    #[inline(always)]
    pub fn cr_irrx_end_th(&self) -> CrIrrxEndThR {
        CrIrrxEndThR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IRRX data threshold"]
    #[inline(always)]
    pub fn cr_irrx_data_th(&mut self) -> CrIrrxDataThW<'_, IrrxPwConfigSpec> {
        CrIrrxDataThW::new(self, 0)
    }
    #[doc = "Bits 16:31 - IRRX end-of-transfer threshold"]
    #[inline(always)]
    pub fn cr_irrx_end_th(&mut self) -> CrIrrxEndThW<'_, IrrxPwConfigSpec> {
        CrIrrxEndThW::new(self, 16)
    }
}
#[doc = "IRRX Pulse Width Threshold Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`irrx_pw_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irrx_pw_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrrxPwConfigSpec;
impl crate::RegisterSpec for IrrxPwConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irrx_pw_config::R`](R) reader structure"]
impl crate::Readable for IrrxPwConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`irrx_pw_config::W`](W) writer structure"]
impl crate::Writable for IrrxPwConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRRX_PW_CONFIG to value 0x2327_0d47"]
impl crate::Resettable for IrrxPwConfigSpec {
    const RESET_VALUE: u32 = 0x2327_0d47;
}
