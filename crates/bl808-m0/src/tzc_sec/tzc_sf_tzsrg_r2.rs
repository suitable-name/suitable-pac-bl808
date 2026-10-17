#[doc = "Register `TZC_SF_TZSRG_R2` reader"]
pub type R = crate::R<TzcSfTzsrgR2Spec>;
#[doc = "Register `TZC_SF_TZSRG_R2` writer"]
pub type W = crate::W<TzcSfTzsrgR2Spec>;
#[doc = "Field `tzc_sf_tzsrg_r2_end` reader - End address boundary (16-bit offset)"]
pub type TzcSfTzsrgR2EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_sf_tzsrg_r2_end` writer - End address boundary (16-bit offset)"]
pub type TzcSfTzsrgR2EndW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `tzc_sf_tzsrg_r2_start` reader - Start address boundary (16-bit offset)"]
pub type TzcSfTzsrgR2StartR = crate::FieldReader<u16>;
#[doc = "Field `tzc_sf_tzsrg_r2_start` writer - Start address boundary (16-bit offset)"]
pub type TzcSfTzsrgR2StartW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - End address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_end(&self) -> TzcSfTzsrgR2EndR {
        TzcSfTzsrgR2EndR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Start address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_start(&self) -> TzcSfTzsrgR2StartR {
        TzcSfTzsrgR2StartR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - End address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_end(&mut self) -> TzcSfTzsrgR2EndW<'_, TzcSfTzsrgR2Spec> {
        TzcSfTzsrgR2EndW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Start address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r2_start(&mut self) -> TzcSfTzsrgR2StartW<'_, TzcSfTzsrgR2Spec> {
        TzcSfTzsrgR2StartW::new(self, 16)
    }
}
#[doc = "Serial Flash Security Region 2 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_r2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_sf_tzsrg_r2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcSfTzsrgR2Spec;
impl crate::RegisterSpec for TzcSfTzsrgR2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_sf_tzsrg_r2::R`](R) reader structure"]
impl crate::Readable for TzcSfTzsrgR2Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_sf_tzsrg_r2::W`](W) writer structure"]
impl crate::Writable for TzcSfTzsrgR2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_SF_TZSRG_R2 to value 0xffff"]
impl crate::Resettable for TzcSfTzsrgR2Spec {
    const RESET_VALUE: u32 = 0xffff;
}
