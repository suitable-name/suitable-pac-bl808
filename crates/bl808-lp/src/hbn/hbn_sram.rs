#[doc = "Register `HBN_SRAM` reader"]
pub type R = crate::R<HbnSramSpec>;
#[doc = "Register `HBN_SRAM` writer"]
pub type W = crate::W<HbnSramSpec>;
#[doc = "Field `retram_ret` reader - "]
pub type RetramRetR = crate::BitReader;
#[doc = "Field `retram_ret` writer - "]
pub type RetramRetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `retram_slp` reader - "]
pub type RetramSlpR = crate::BitReader;
#[doc = "Field `retram_slp` writer - "]
pub type RetramSlpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn retram_ret(&self) -> RetramRetR {
        RetramRetR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn retram_slp(&self) -> RetramSlpR {
        RetramSlpR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn retram_ret(&mut self) -> RetramRetW<'_, HbnSramSpec> {
        RetramRetW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn retram_slp(&mut self) -> RetramSlpW<'_, HbnSramSpec> {
        RetramSlpW::new(self, 7)
    }
}
#[doc = "SRAM Retention Control\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_sram::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_sram::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HbnSramSpec;
impl crate::RegisterSpec for HbnSramSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbn_sram::R`](R) reader structure"]
impl crate::Readable for HbnSramSpec {}
#[doc = "`write(|w| ..)` method takes [`hbn_sram::W`](W) writer structure"]
impl crate::Writable for HbnSramSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HBN_SRAM to value 0"]
impl crate::Resettable for HbnSramSpec {}
