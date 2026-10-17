#[doc = "Register `SF3_IF_IO_DLY_2` reader"]
pub type R = crate::R<Sf3IfIoDly2Spec>;
#[doc = "Register `SF3_IF_IO_DLY_2` writer"]
pub type W = crate::W<Sf3IfIoDly2Spec>;
#[doc = "Field `sf3_io_1_oe_dly_sel` reader - "]
pub type Sf3Io1OeDlySelR = crate::FieldReader;
#[doc = "Field `sf3_io_1_oe_dly_sel` writer - "]
pub type Sf3Io1OeDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf3_io_1_di_dly_sel` reader - "]
pub type Sf3Io1DiDlySelR = crate::FieldReader;
#[doc = "Field `sf3_io_1_di_dly_sel` writer - "]
pub type Sf3Io1DiDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf3_io_1_do_dly_sel` reader - "]
pub type Sf3Io1DoDlySelR = crate::FieldReader;
#[doc = "Field `sf3_io_1_do_dly_sel` writer - "]
pub type Sf3Io1DoDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf3_io_1_oe_dly_sel(&self) -> Sf3Io1OeDlySelR {
        Sf3Io1OeDlySelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf3_io_1_di_dly_sel(&self) -> Sf3Io1DiDlySelR {
        Sf3Io1DiDlySelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sf3_io_1_do_dly_sel(&self) -> Sf3Io1DoDlySelR {
        Sf3Io1DoDlySelR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf3_io_1_oe_dly_sel(&mut self) -> Sf3Io1OeDlySelW<'_, Sf3IfIoDly2Spec> {
        Sf3Io1OeDlySelW::new(self, 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf3_io_1_di_dly_sel(&mut self) -> Sf3Io1DiDlySelW<'_, Sf3IfIoDly2Spec> {
        Sf3Io1DiDlySelW::new(self, 8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sf3_io_1_do_dly_sel(&mut self) -> Sf3Io1DoDlySelW<'_, Sf3IfIoDly2Spec> {
        Sf3Io1DoDlySelW::new(self, 16)
    }
}
#[doc = "SF3 IO 1 Delay Tuning\n\nYou can [`read`](crate::Reg::read) this register and get [`sf3_if_io_dly_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf3_if_io_dly_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sf3IfIoDly2Spec;
impl crate::RegisterSpec for Sf3IfIoDly2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf3_if_io_dly_2::R`](R) reader structure"]
impl crate::Readable for Sf3IfIoDly2Spec {}
#[doc = "`write(|w| ..)` method takes [`sf3_if_io_dly_2::W`](W) writer structure"]
impl crate::Writable for Sf3IfIoDly2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF3_IF_IO_DLY_2 to value 0"]
impl crate::Resettable for Sf3IfIoDly2Spec {}
