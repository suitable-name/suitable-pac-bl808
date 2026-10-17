#[doc = "Register `HBN_PIR_VTH` reader"]
pub type R = crate::R<HbnPirVthSpec>;
#[doc = "Register `HBN_PIR_VTH` writer"]
pub type W = crate::W<HbnPirVthSpec>;
#[doc = "Field `pir_vth` reader - "]
pub type PirVthR = crate::FieldReader<u16>;
#[doc = "Field `pir_vth` writer - "]
pub type PirVthW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn pir_vth(&self) -> PirVthR {
        PirVthR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn pir_vth(&mut self) -> PirVthW<'_, HbnPirVthSpec> {
        PirVthW::new(self, 0)
    }
}
#[doc = "PIR Sensor Voltage Threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`hbn_pir_vth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hbn_pir_vth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HbnPirVthSpec;
impl crate::RegisterSpec for HbnPirVthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hbn_pir_vth::R`](R) reader structure"]
impl crate::Readable for HbnPirVthSpec {}
#[doc = "`write(|w| ..)` method takes [`hbn_pir_vth::W`](W) writer structure"]
impl crate::Writable for HbnPirVthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HBN_PIR_VTH to value 0x03ff"]
impl crate::Resettable for HbnPirVthSpec {
    const RESET_VALUE: u32 = 0x03ff;
}
