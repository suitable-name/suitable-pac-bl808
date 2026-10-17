#[doc = "Register `SF_IF_IO_DLY_1` reader"]
pub type R = crate::R<SfIfIoDly1Spec>;
#[doc = "Register `SF_IF_IO_DLY_1` writer"]
pub type W = crate::W<SfIfIoDly1Spec>;
#[doc = "Field `sf_io_0_oe_dly_sel` reader - "]
pub type SfIo0OeDlySelR = crate::FieldReader;
#[doc = "Field `sf_io_0_oe_dly_sel` writer - "]
pub type SfIo0OeDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf_io_0_di_dly_sel` reader - "]
pub type SfIo0DiDlySelR = crate::FieldReader;
#[doc = "Field `sf_io_0_di_dly_sel` writer - "]
pub type SfIo0DiDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf_io_0_do_dly_sel` reader - "]
pub type SfIo0DoDlySelR = crate::FieldReader;
#[doc = "Field `sf_io_0_do_dly_sel` writer - "]
pub type SfIo0DoDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf_io_0_oe_dly_sel(&self) -> SfIo0OeDlySelR {
        SfIo0OeDlySelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf_io_0_di_dly_sel(&self) -> SfIo0DiDlySelR {
        SfIo0DiDlySelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sf_io_0_do_dly_sel(&self) -> SfIo0DoDlySelR {
        SfIo0DoDlySelR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf_io_0_oe_dly_sel(&mut self) -> SfIo0OeDlySelW<'_, SfIfIoDly1Spec> {
        SfIo0OeDlySelW::new(self, 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf_io_0_di_dly_sel(&mut self) -> SfIo0DiDlySelW<'_, SfIfIoDly1Spec> {
        SfIo0DiDlySelW::new(self, 8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sf_io_0_do_dly_sel(&mut self) -> SfIo0DoDlySelW<'_, SfIfIoDly1Spec> {
        SfIo0DoDlySelW::new(self, 16)
    }
}
#[doc = "SF IO 0 Delay Tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_io_dly_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_io_dly_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfIfIoDly1Spec;
impl crate::RegisterSpec for SfIfIoDly1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_if_io_dly_1::R`](R) reader structure"]
impl crate::Readable for SfIfIoDly1Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_if_io_dly_1::W`](W) writer structure"]
impl crate::Writable for SfIfIoDly1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_IF_IO_DLY_1 to value 0"]
impl crate::Resettable for SfIfIoDly1Spec {}
