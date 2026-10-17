#[doc = "Register `SF3_IF_IO_DLY_0` reader"]
pub type R = crate::R<Sf3IfIoDly0Spec>;
#[doc = "Register `SF3_IF_IO_DLY_0` writer"]
pub type W = crate::W<Sf3IfIoDly0Spec>;
#[doc = "Field `sf3_cs_dly_sel` reader - "]
pub type Sf3CsDlySelR = crate::FieldReader;
#[doc = "Field `sf3_cs_dly_sel` writer - "]
pub type Sf3CsDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf3_cs2_dly_sel` reader - "]
pub type Sf3Cs2DlySelR = crate::FieldReader;
#[doc = "Field `sf3_cs2_dly_sel` writer - "]
pub type Sf3Cs2DlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf3_clk_out_dly_sel` reader - "]
pub type Sf3ClkOutDlySelR = crate::FieldReader;
#[doc = "Field `sf3_clk_out_dly_sel` writer - "]
pub type Sf3ClkOutDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf3_dqs_oe_dly_sel` reader - "]
pub type Sf3DqsOeDlySelR = crate::FieldReader;
#[doc = "Field `sf3_dqs_oe_dly_sel` writer - "]
pub type Sf3DqsOeDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf3_dqs_di_dly_sel` reader - "]
pub type Sf3DqsDiDlySelR = crate::FieldReader;
#[doc = "Field `sf3_dqs_di_dly_sel` writer - "]
pub type Sf3DqsDiDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf3_dqs_do_dly_sel` reader - "]
pub type Sf3DqsDoDlySelR = crate::FieldReader;
#[doc = "Field `sf3_dqs_do_dly_sel` writer - "]
pub type Sf3DqsDoDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf3_cs_dly_sel(&self) -> Sf3CsDlySelR {
        Sf3CsDlySelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sf3_cs2_dly_sel(&self) -> Sf3Cs2DlySelR {
        Sf3Cs2DlySelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf3_clk_out_dly_sel(&self) -> Sf3ClkOutDlySelR {
        Sf3ClkOutDlySelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn sf3_dqs_oe_dly_sel(&self) -> Sf3DqsOeDlySelR {
        Sf3DqsOeDlySelR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn sf3_dqs_di_dly_sel(&self) -> Sf3DqsDiDlySelR {
        Sf3DqsDiDlySelR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn sf3_dqs_do_dly_sel(&self) -> Sf3DqsDoDlySelR {
        Sf3DqsDoDlySelR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf3_cs_dly_sel(&mut self) -> Sf3CsDlySelW<'_, Sf3IfIoDly0Spec> {
        Sf3CsDlySelW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sf3_cs2_dly_sel(&mut self) -> Sf3Cs2DlySelW<'_, Sf3IfIoDly0Spec> {
        Sf3Cs2DlySelW::new(self, 2)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf3_clk_out_dly_sel(&mut self) -> Sf3ClkOutDlySelW<'_, Sf3IfIoDly0Spec> {
        Sf3ClkOutDlySelW::new(self, 8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn sf3_dqs_oe_dly_sel(&mut self) -> Sf3DqsOeDlySelW<'_, Sf3IfIoDly0Spec> {
        Sf3DqsOeDlySelW::new(self, 26)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn sf3_dqs_di_dly_sel(&mut self) -> Sf3DqsDiDlySelW<'_, Sf3IfIoDly0Spec> {
        Sf3DqsDiDlySelW::new(self, 28)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn sf3_dqs_do_dly_sel(&mut self) -> Sf3DqsDoDlySelW<'_, Sf3IfIoDly0Spec> {
        Sf3DqsDoDlySelW::new(self, 30)
    }
}
#[doc = "SF3 IO Delay Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf3_if_io_dly_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf3_if_io_dly_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sf3IfIoDly0Spec;
impl crate::RegisterSpec for Sf3IfIoDly0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf3_if_io_dly_0::R`](R) reader structure"]
impl crate::Readable for Sf3IfIoDly0Spec {}
#[doc = "`write(|w| ..)` method takes [`sf3_if_io_dly_0::W`](W) writer structure"]
impl crate::Writable for Sf3IfIoDly0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF3_IF_IO_DLY_0 to value 0"]
impl crate::Resettable for Sf3IfIoDly0Spec {}
