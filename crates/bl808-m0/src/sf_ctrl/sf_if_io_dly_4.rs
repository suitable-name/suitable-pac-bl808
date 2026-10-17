#[doc = "Register `SF_IF_IO_DLY_4` reader"]
pub type R = crate::R<SfIfIoDly4Spec>;
#[doc = "Register `SF_IF_IO_DLY_4` writer"]
pub type W = crate::W<SfIfIoDly4Spec>;
#[doc = "Field `sf_io_3_oe_dly_sel` reader - "]
pub type SfIo3OeDlySelR = crate::FieldReader;
#[doc = "Field `sf_io_3_oe_dly_sel` writer - "]
pub type SfIo3OeDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf_io_3_di_dly_sel` reader - "]
pub type SfIo3DiDlySelR = crate::FieldReader;
#[doc = "Field `sf_io_3_di_dly_sel` writer - "]
pub type SfIo3DiDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf_io_3_do_dly_sel` reader - "]
pub type SfIo3DoDlySelR = crate::FieldReader;
#[doc = "Field `sf_io_3_do_dly_sel` writer - "]
pub type SfIo3DoDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf_io_3_oe_dly_sel(&self) -> SfIo3OeDlySelR {
        SfIo3OeDlySelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf_io_3_di_dly_sel(&self) -> SfIo3DiDlySelR {
        SfIo3DiDlySelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sf_io_3_do_dly_sel(&self) -> SfIo3DoDlySelR {
        SfIo3DoDlySelR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf_io_3_oe_dly_sel(&mut self) -> SfIo3OeDlySelW<'_, SfIfIoDly4Spec> {
        SfIo3OeDlySelW::new(self, 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf_io_3_di_dly_sel(&mut self) -> SfIo3DiDlySelW<'_, SfIfIoDly4Spec> {
        SfIo3DiDlySelW::new(self, 8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sf_io_3_do_dly_sel(&mut self) -> SfIo3DoDlySelW<'_, SfIfIoDly4Spec> {
        SfIo3DoDlySelW::new(self, 16)
    }
}
#[doc = "SF IO 3 Delay Tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_io_dly_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_io_dly_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfIfIoDly4Spec;
impl crate::RegisterSpec for SfIfIoDly4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_if_io_dly_4::R`](R) reader structure"]
impl crate::Readable for SfIfIoDly4Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_if_io_dly_4::W`](W) writer structure"]
impl crate::Writable for SfIfIoDly4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_IF_IO_DLY_4 to value 0"]
impl crate::Resettable for SfIfIoDly4Spec {}
