#[doc = "Register `SF3_IF_IO_DLY_4` reader"]
pub type R = crate::R<Sf3IfIoDly4Spec>;
#[doc = "Register `SF3_IF_IO_DLY_4` writer"]
pub type W = crate::W<Sf3IfIoDly4Spec>;
#[doc = "Field `sf3_io_3_oe_dly_sel` reader - "]
pub type Sf3Io3OeDlySelR = crate::FieldReader;
#[doc = "Field `sf3_io_3_oe_dly_sel` writer - "]
pub type Sf3Io3OeDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf3_io_3_di_dly_sel` reader - "]
pub type Sf3Io3DiDlySelR = crate::FieldReader;
#[doc = "Field `sf3_io_3_di_dly_sel` writer - "]
pub type Sf3Io3DiDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf3_io_3_do_dly_sel` reader - "]
pub type Sf3Io3DoDlySelR = crate::FieldReader;
#[doc = "Field `sf3_io_3_do_dly_sel` writer - "]
pub type Sf3Io3DoDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf3_io_3_oe_dly_sel(&self) -> Sf3Io3OeDlySelR {
        Sf3Io3OeDlySelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf3_io_3_di_dly_sel(&self) -> Sf3Io3DiDlySelR {
        Sf3Io3DiDlySelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sf3_io_3_do_dly_sel(&self) -> Sf3Io3DoDlySelR {
        Sf3Io3DoDlySelR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf3_io_3_oe_dly_sel(&mut self) -> Sf3Io3OeDlySelW<'_, Sf3IfIoDly4Spec> {
        Sf3Io3OeDlySelW::new(self, 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf3_io_3_di_dly_sel(&mut self) -> Sf3Io3DiDlySelW<'_, Sf3IfIoDly4Spec> {
        Sf3Io3DiDlySelW::new(self, 8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sf3_io_3_do_dly_sel(&mut self) -> Sf3Io3DoDlySelW<'_, Sf3IfIoDly4Spec> {
        Sf3Io3DoDlySelW::new(self, 16)
    }
}
#[doc = "SF3 IO 3 Delay Tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`sf3_if_io_dly_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf3_if_io_dly_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sf3IfIoDly4Spec;
impl crate::RegisterSpec for Sf3IfIoDly4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf3_if_io_dly_4::R`](R) reader structure"]
impl crate::Readable for Sf3IfIoDly4Spec {}
#[doc = "`write(|w| ..)` method takes [`sf3_if_io_dly_4::W`](W) writer structure"]
impl crate::Writable for Sf3IfIoDly4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF3_IF_IO_DLY_4 to value 0"]
impl crate::Resettable for Sf3IfIoDly4Spec {}
