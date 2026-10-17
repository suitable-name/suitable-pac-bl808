#[doc = "Register `TZC_SF_TZSRG_R3` reader"]
pub type R = crate::R<TzcSfTzsrgR3Spec>;
#[doc = "Register `TZC_SF_TZSRG_R3` writer"]
pub type W = crate::W<TzcSfTzsrgR3Spec>;
#[doc = "Field `tzc_sf_tzsrg_r3_end` reader - End address boundary (16-bit offset)"]
pub type TzcSfTzsrgR3EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_sf_tzsrg_r3_end` writer - End address boundary (16-bit offset)"]
pub type TzcSfTzsrgR3EndW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `tzc_sf_tzsrg_r3_start` reader - Start address boundary (16-bit offset)"]
pub type TzcSfTzsrgR3StartR = crate::FieldReader<u16>;
#[doc = "Field `tzc_sf_tzsrg_r3_start` writer - Start address boundary (16-bit offset)"]
pub type TzcSfTzsrgR3StartW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - End address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_end(&self) -> TzcSfTzsrgR3EndR {
        TzcSfTzsrgR3EndR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Start address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_start(&self) -> TzcSfTzsrgR3StartR {
        TzcSfTzsrgR3StartR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - End address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_end(&mut self) -> TzcSfTzsrgR3EndW<'_, TzcSfTzsrgR3Spec> {
        TzcSfTzsrgR3EndW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Start address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r3_start(&mut self) -> TzcSfTzsrgR3StartW<'_, TzcSfTzsrgR3Spec> {
        TzcSfTzsrgR3StartW::new(self, 16)
    }
}
#[doc = "Serial Flash Security Region 3 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_r3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_sf_tzsrg_r3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcSfTzsrgR3Spec;
impl crate::RegisterSpec for TzcSfTzsrgR3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_sf_tzsrg_r3::R`](R) reader structure"]
impl crate::Readable for TzcSfTzsrgR3Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_sf_tzsrg_r3::W`](W) writer structure"]
impl crate::Writable for TzcSfTzsrgR3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_SF_TZSRG_R3 to value 0xffff"]
impl crate::Resettable for TzcSfTzsrgR3Spec {
    const RESET_VALUE: u32 = 0xffff;
}
