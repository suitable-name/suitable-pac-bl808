#[doc = "Register `PIX_DATA_STEP` reader"]
pub type R = crate::R<PixDataStepSpec>;
#[doc = "Register `PIX_DATA_STEP` writer"]
pub type W = crate::W<PixDataStepSpec>;
#[doc = "Field `cr_data_step` reader - Pixel data step size (affects sub-sampling/interleaving)"]
pub type CrDataStepR = crate::FieldReader;
#[doc = "Field `cr_data_step` writer - Pixel data step size (affects sub-sampling/interleaving)"]
pub type CrDataStepW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Pixel data step size (affects sub-sampling/interleaving)"]
    #[inline(always)]
    pub fn cr_data_step(&self) -> CrDataStepR {
        CrDataStepR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Pixel data step size (affects sub-sampling/interleaving)"]
    #[inline(always)]
    pub fn cr_data_step(&mut self) -> CrDataStepW<'_, PixDataStepSpec> {
        CrDataStepW::new(self, 0)
    }
}
#[doc = "Pixel Data Step/Stride Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pix_data_step::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pix_data_step::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PixDataStepSpec;
impl crate::RegisterSpec for PixDataStepSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pix_data_step::R`](R) reader structure"]
impl crate::Readable for PixDataStepSpec {}
#[doc = "`write(|w| ..)` method takes [`pix_data_step::W`](W) writer structure"]
impl crate::Writable for PixDataStepSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PIX_DATA_STEP to value 0x01"]
impl crate::Resettable for PixDataStepSpec {
    const RESET_VALUE: u32 = 0x01;
}
