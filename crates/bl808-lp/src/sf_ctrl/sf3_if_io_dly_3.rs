#[doc = "Register `SF3_IF_IO_DLY_3` reader"]
pub type R = crate::R<Sf3IfIoDly3Spec>;
#[doc = "Register `SF3_IF_IO_DLY_3` writer"]
pub type W = crate::W<Sf3IfIoDly3Spec>;
#[doc = "Field `sf3_io_2_oe_dly_sel` reader - "]
pub type Sf3Io2OeDlySelR = crate::FieldReader;
#[doc = "Field `sf3_io_2_oe_dly_sel` writer - "]
pub type Sf3Io2OeDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf3_io_2_di_dly_sel` reader - "]
pub type Sf3Io2DiDlySelR = crate::FieldReader;
#[doc = "Field `sf3_io_2_di_dly_sel` writer - "]
pub type Sf3Io2DiDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf3_io_2_do_dly_sel` reader - "]
pub type Sf3Io2DoDlySelR = crate::FieldReader;
#[doc = "Field `sf3_io_2_do_dly_sel` writer - "]
pub type Sf3Io2DoDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf3_io_2_oe_dly_sel(&self) -> Sf3Io2OeDlySelR {
        Sf3Io2OeDlySelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf3_io_2_di_dly_sel(&self) -> Sf3Io2DiDlySelR {
        Sf3Io2DiDlySelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sf3_io_2_do_dly_sel(&self) -> Sf3Io2DoDlySelR {
        Sf3Io2DoDlySelR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf3_io_2_oe_dly_sel(&mut self) -> Sf3Io2OeDlySelW<'_, Sf3IfIoDly3Spec> {
        Sf3Io2OeDlySelW::new(self, 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf3_io_2_di_dly_sel(&mut self) -> Sf3Io2DiDlySelW<'_, Sf3IfIoDly3Spec> {
        Sf3Io2DiDlySelW::new(self, 8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sf3_io_2_do_dly_sel(&mut self) -> Sf3Io2DoDlySelW<'_, Sf3IfIoDly3Spec> {
        Sf3Io2DoDlySelW::new(self, 16)
    }
}
#[doc = "SF3 IO 2 Delay Tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`sf3_if_io_dly_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf3_if_io_dly_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sf3IfIoDly3Spec;
impl crate::RegisterSpec for Sf3IfIoDly3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf3_if_io_dly_3::R`](R) reader structure"]
impl crate::Readable for Sf3IfIoDly3Spec {}
#[doc = "`write(|w| ..)` method takes [`sf3_if_io_dly_3::W`](W) writer structure"]
impl crate::Writable for Sf3IfIoDly3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF3_IF_IO_DLY_3 to value 0"]
impl crate::Resettable for Sf3IfIoDly3Spec {}
