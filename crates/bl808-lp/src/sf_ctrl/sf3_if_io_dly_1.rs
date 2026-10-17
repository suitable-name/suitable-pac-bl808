#[doc = "Register `SF3_IF_IO_DLY_1` reader"]
pub type R = crate::R<Sf3IfIoDly1Spec>;
#[doc = "Register `SF3_IF_IO_DLY_1` writer"]
pub type W = crate::W<Sf3IfIoDly1Spec>;
#[doc = "Field `sf3_io_0_oe_dly_sel` reader - "]
pub type Sf3Io0OeDlySelR = crate::FieldReader;
#[doc = "Field `sf3_io_0_oe_dly_sel` writer - "]
pub type Sf3Io0OeDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf3_io_0_di_dly_sel` reader - "]
pub type Sf3Io0DiDlySelR = crate::FieldReader;
#[doc = "Field `sf3_io_0_di_dly_sel` writer - "]
pub type Sf3Io0DiDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf3_io_0_do_dly_sel` reader - "]
pub type Sf3Io0DoDlySelR = crate::FieldReader;
#[doc = "Field `sf3_io_0_do_dly_sel` writer - "]
pub type Sf3Io0DoDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf3_io_0_oe_dly_sel(&self) -> Sf3Io0OeDlySelR {
        Sf3Io0OeDlySelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf3_io_0_di_dly_sel(&self) -> Sf3Io0DiDlySelR {
        Sf3Io0DiDlySelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sf3_io_0_do_dly_sel(&self) -> Sf3Io0DoDlySelR {
        Sf3Io0DoDlySelR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf3_io_0_oe_dly_sel(&mut self) -> Sf3Io0OeDlySelW<'_, Sf3IfIoDly1Spec> {
        Sf3Io0OeDlySelW::new(self, 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf3_io_0_di_dly_sel(&mut self) -> Sf3Io0DiDlySelW<'_, Sf3IfIoDly1Spec> {
        Sf3Io0DiDlySelW::new(self, 8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sf3_io_0_do_dly_sel(&mut self) -> Sf3Io0DoDlySelW<'_, Sf3IfIoDly1Spec> {
        Sf3Io0DoDlySelW::new(self, 16)
    }
}
#[doc = "SF3 IO 0 Delay Tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`sf3_if_io_dly_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf3_if_io_dly_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sf3IfIoDly1Spec;
impl crate::RegisterSpec for Sf3IfIoDly1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf3_if_io_dly_1::R`](R) reader structure"]
impl crate::Readable for Sf3IfIoDly1Spec {}
#[doc = "`write(|w| ..)` method takes [`sf3_if_io_dly_1::W`](W) writer structure"]
impl crate::Writable for Sf3IfIoDly1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF3_IF_IO_DLY_1 to value 0"]
impl crate::Resettable for Sf3IfIoDly1Spec {}
