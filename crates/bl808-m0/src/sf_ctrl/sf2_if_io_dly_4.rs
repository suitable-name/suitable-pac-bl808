#[doc = "Register `SF2_IF_IO_DLY_4` reader"]
pub type R = crate::R<Sf2IfIoDly4Spec>;
#[doc = "Register `SF2_IF_IO_DLY_4` writer"]
pub type W = crate::W<Sf2IfIoDly4Spec>;
#[doc = "Field `sf2_io_3_oe_dly_sel` reader - "]
pub type Sf2Io3OeDlySelR = crate::FieldReader;
#[doc = "Field `sf2_io_3_oe_dly_sel` writer - "]
pub type Sf2Io3OeDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf2_io_3_di_dly_sel` reader - "]
pub type Sf2Io3DiDlySelR = crate::FieldReader;
#[doc = "Field `sf2_io_3_di_dly_sel` writer - "]
pub type Sf2Io3DiDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf2_io_3_do_dly_sel` reader - "]
pub type Sf2Io3DoDlySelR = crate::FieldReader;
#[doc = "Field `sf2_io_3_do_dly_sel` writer - "]
pub type Sf2Io3DoDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf2_io_3_oe_dly_sel(&self) -> Sf2Io3OeDlySelR {
        Sf2Io3OeDlySelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf2_io_3_di_dly_sel(&self) -> Sf2Io3DiDlySelR {
        Sf2Io3DiDlySelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sf2_io_3_do_dly_sel(&self) -> Sf2Io3DoDlySelR {
        Sf2Io3DoDlySelR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf2_io_3_oe_dly_sel(&mut self) -> Sf2Io3OeDlySelW<'_, Sf2IfIoDly4Spec> {
        Sf2Io3OeDlySelW::new(self, 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf2_io_3_di_dly_sel(&mut self) -> Sf2Io3DiDlySelW<'_, Sf2IfIoDly4Spec> {
        Sf2Io3DiDlySelW::new(self, 8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sf2_io_3_do_dly_sel(&mut self) -> Sf2Io3DoDlySelW<'_, Sf2IfIoDly4Spec> {
        Sf2Io3DoDlySelW::new(self, 16)
    }
}
#[doc = "SF2 IO 3 Delay Tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`sf2_if_io_dly_4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf2_if_io_dly_4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sf2IfIoDly4Spec;
impl crate::RegisterSpec for Sf2IfIoDly4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf2_if_io_dly_4::R`](R) reader structure"]
impl crate::Readable for Sf2IfIoDly4Spec {}
#[doc = "`write(|w| ..)` method takes [`sf2_if_io_dly_4::W`](W) writer structure"]
impl crate::Writable for Sf2IfIoDly4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF2_IF_IO_DLY_4 to value 0"]
impl crate::Resettable for Sf2IfIoDly4Spec {}
