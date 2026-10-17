#[doc = "Register `HBN_PIR_INTERVAL` reader"]
pub type R = crate::R<HbnPirIntervalSpec>;
#[doc = "Register `HBN_PIR_INTERVAL` writer"]
pub type W = crate::W<HbnPirIntervalSpec>;
#[doc = "Field `pir_interval` reader - "]
pub type PirIntervalR = crate::FieldReader<u16>;
#[doc = "Field `pir_interval` writer - "]
pub type PirIntervalW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn pir_interval(&self) -> PirIntervalR {
        PirIntervalR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn pir_interval(&mut self) -> PirIntervalW<'_, HbnPirIntervalSpec> {
        PirIntervalW::new(self, 0)
    }
}
#[doc = "PIR Check Interval\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_pir_interval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_pir_interval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HbnPirIntervalSpec;
impl crate::RegisterSpec for HbnPirIntervalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbn_pir_interval::R`](R) reader structure"]
impl crate::Readable for HbnPirIntervalSpec {}
#[doc = "`write(|w| ..)` method takes [`hbn_pir_interval::W`](W) writer structure"]
impl crate::Writable for HbnPirIntervalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HBN_PIR_INTERVAL to value 0x0a3d"]
impl crate::Resettable for HbnPirIntervalSpec {
    const RESET_VALUE: u32 = 0x0a3d;
}
