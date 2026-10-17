#[doc = "Register `TZC_SF_TZSRG_R1` reader"]
pub type R = crate::R<TzcSfTzsrgR1Spec>;
#[doc = "Register `TZC_SF_TZSRG_R1` writer"]
pub type W = crate::W<TzcSfTzsrgR1Spec>;
#[doc = "Field `tzc_sf_tzsrg_r1_end` reader - End address boundary (16-bit offset)"]
pub type TzcSfTzsrgR1EndR = crate::FieldReader<u16>;
#[doc = "Field `tzc_sf_tzsrg_r1_end` writer - End address boundary (16-bit offset)"]
pub type TzcSfTzsrgR1EndW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `tzc_sf_tzsrg_r1_start` reader - Start address boundary (16-bit offset)"]
pub type TzcSfTzsrgR1StartR = crate::FieldReader<u16>;
#[doc = "Field `tzc_sf_tzsrg_r1_start` writer - Start address boundary (16-bit offset)"]
pub type TzcSfTzsrgR1StartW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - End address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_end(&self) -> TzcSfTzsrgR1EndR {
        TzcSfTzsrgR1EndR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Start address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_start(&self) -> TzcSfTzsrgR1StartR {
        TzcSfTzsrgR1StartR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - End address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_end(&mut self) -> TzcSfTzsrgR1EndW<'_, TzcSfTzsrgR1Spec> {
        TzcSfTzsrgR1EndW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Start address boundary (16-bit offset)"]
    #[inline(always)]
    pub fn tzc_sf_tzsrg_r1_start(&mut self) -> TzcSfTzsrgR1StartW<'_, TzcSfTzsrgR1Spec> {
        TzcSfTzsrgR1StartW::new(self, 16)
    }
}
#[doc = "Serial Flash Security Region 1 Start/End Address\n\nYou can [`read`](crate::Reg::read) this register and get [`tzc_sf_tzsrg_r1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzc_sf_tzsrg_r1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzcSfTzsrgR1Spec;
impl crate::RegisterSpec for TzcSfTzsrgR1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_sf_tzsrg_r1::R`](R) reader structure"]
impl crate::Readable for TzcSfTzsrgR1Spec {}
#[doc = "`write(|w| ..)` method takes [`tzc_sf_tzsrg_r1::W`](W) writer structure"]
impl crate::Writable for TzcSfTzsrgR1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZC_SF_TZSRG_R1 to value 0xffff"]
impl crate::Resettable for TzcSfTzsrgR1Spec {
    const RESET_VALUE: u32 = 0xffff;
}
