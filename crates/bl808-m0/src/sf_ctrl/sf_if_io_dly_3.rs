#[doc = "Register `SF_IF_IO_DLY_3` reader"]
pub type R = crate::R<SfIfIoDly3Spec>;
#[doc = "Register `SF_IF_IO_DLY_3` writer"]
pub type W = crate::W<SfIfIoDly3Spec>;
#[doc = "Field `sf_io_2_oe_dly_sel` reader - "]
pub type SfIo2OeDlySelR = crate::FieldReader;
#[doc = "Field `sf_io_2_oe_dly_sel` writer - "]
pub type SfIo2OeDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf_io_2_di_dly_sel` reader - "]
pub type SfIo2DiDlySelR = crate::FieldReader;
#[doc = "Field `sf_io_2_di_dly_sel` writer - "]
pub type SfIo2DiDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf_io_2_do_dly_sel` reader - "]
pub type SfIo2DoDlySelR = crate::FieldReader;
#[doc = "Field `sf_io_2_do_dly_sel` writer - "]
pub type SfIo2DoDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf_io_2_oe_dly_sel(&self) -> SfIo2OeDlySelR {
        SfIo2OeDlySelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf_io_2_di_dly_sel(&self) -> SfIo2DiDlySelR {
        SfIo2DiDlySelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sf_io_2_do_dly_sel(&self) -> SfIo2DoDlySelR {
        SfIo2DoDlySelR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf_io_2_oe_dly_sel(&mut self) -> SfIo2OeDlySelW<'_, SfIfIoDly3Spec> {
        SfIo2OeDlySelW::new(self, 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf_io_2_di_dly_sel(&mut self) -> SfIo2DiDlySelW<'_, SfIfIoDly3Spec> {
        SfIo2DiDlySelW::new(self, 8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sf_io_2_do_dly_sel(&mut self) -> SfIo2DoDlySelW<'_, SfIfIoDly3Spec> {
        SfIo2DoDlySelW::new(self, 16)
    }
}
#[doc = "SF IO 2 Delay Tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_io_dly_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_io_dly_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfIfIoDly3Spec;
impl crate::RegisterSpec for SfIfIoDly3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_if_io_dly_3::R`](R) reader structure"]
impl crate::Readable for SfIfIoDly3Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_if_io_dly_3::W`](W) writer structure"]
impl crate::Writable for SfIfIoDly3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_IF_IO_DLY_3 to value 0"]
impl crate::Resettable for SfIfIoDly3Spec {}
