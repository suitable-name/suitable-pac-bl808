#[doc = "Register `SF_IF_IO_DLY_2` reader"]
pub type R = crate::R<SfIfIoDly2Spec>;
#[doc = "Register `SF_IF_IO_DLY_2` writer"]
pub type W = crate::W<SfIfIoDly2Spec>;
#[doc = "Field `sf_io_1_oe_dly_sel` reader - "]
pub type SfIo1OeDlySelR = crate::FieldReader;
#[doc = "Field `sf_io_1_oe_dly_sel` writer - "]
pub type SfIo1OeDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf_io_1_di_dly_sel` reader - "]
pub type SfIo1DiDlySelR = crate::FieldReader;
#[doc = "Field `sf_io_1_di_dly_sel` writer - "]
pub type SfIo1DiDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf_io_1_do_dly_sel` reader - "]
pub type SfIo1DoDlySelR = crate::FieldReader;
#[doc = "Field `sf_io_1_do_dly_sel` writer - "]
pub type SfIo1DoDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf_io_1_oe_dly_sel(&self) -> SfIo1OeDlySelR {
        SfIo1OeDlySelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf_io_1_di_dly_sel(&self) -> SfIo1DiDlySelR {
        SfIo1DiDlySelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sf_io_1_do_dly_sel(&self) -> SfIo1DoDlySelR {
        SfIo1DoDlySelR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf_io_1_oe_dly_sel(&mut self) -> SfIo1OeDlySelW<'_, SfIfIoDly2Spec> {
        SfIo1OeDlySelW::new(self, 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf_io_1_di_dly_sel(&mut self) -> SfIo1DiDlySelW<'_, SfIfIoDly2Spec> {
        SfIo1DiDlySelW::new(self, 8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sf_io_1_do_dly_sel(&mut self) -> SfIo1DoDlySelW<'_, SfIfIoDly2Spec> {
        SfIo1DoDlySelW::new(self, 16)
    }
}
#[doc = "SF IO 1 Delay Tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_io_dly_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_io_dly_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfIfIoDly2Spec;
impl crate::RegisterSpec for SfIfIoDly2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_if_io_dly_2::R`](R) reader structure"]
impl crate::Readable for SfIfIoDly2Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_if_io_dly_2::W`](W) writer structure"]
impl crate::Writable for SfIfIoDly2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_IF_IO_DLY_2 to value 0"]
impl crate::Resettable for SfIfIoDly2Spec {}
