#[doc = "Register `SF2_IF_IO_DLY_0` reader"]
pub type R = crate::R<Sf2IfIoDly0Spec>;
#[doc = "Register `SF2_IF_IO_DLY_0` writer"]
pub type W = crate::W<Sf2IfIoDly0Spec>;
#[doc = "Field `sf2_cs_dly_sel` reader - "]
pub type Sf2CsDlySelR = crate::FieldReader;
#[doc = "Field `sf2_cs_dly_sel` writer - "]
pub type Sf2CsDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf2_cs2_dly_sel` reader - "]
pub type Sf2Cs2DlySelR = crate::FieldReader;
#[doc = "Field `sf2_cs2_dly_sel` writer - "]
pub type Sf2Cs2DlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf2_clk_out_dly_sel` reader - "]
pub type Sf2ClkOutDlySelR = crate::FieldReader;
#[doc = "Field `sf2_clk_out_dly_sel` writer - "]
pub type Sf2ClkOutDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf2_dqs_oe_dly_sel` reader - "]
pub type Sf2DqsOeDlySelR = crate::FieldReader;
#[doc = "Field `sf2_dqs_oe_dly_sel` writer - "]
pub type Sf2DqsOeDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf2_dqs_di_dly_sel` reader - "]
pub type Sf2DqsDiDlySelR = crate::FieldReader;
#[doc = "Field `sf2_dqs_di_dly_sel` writer - "]
pub type Sf2DqsDiDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf2_dqs_do_dly_sel` reader - "]
pub type Sf2DqsDoDlySelR = crate::FieldReader;
#[doc = "Field `sf2_dqs_do_dly_sel` writer - "]
pub type Sf2DqsDoDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf2_cs_dly_sel(&self) -> Sf2CsDlySelR {
        Sf2CsDlySelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sf2_cs2_dly_sel(&self) -> Sf2Cs2DlySelR {
        Sf2Cs2DlySelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf2_clk_out_dly_sel(&self) -> Sf2ClkOutDlySelR {
        Sf2ClkOutDlySelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn sf2_dqs_oe_dly_sel(&self) -> Sf2DqsOeDlySelR {
        Sf2DqsOeDlySelR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn sf2_dqs_di_dly_sel(&self) -> Sf2DqsDiDlySelR {
        Sf2DqsDiDlySelR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn sf2_dqs_do_dly_sel(&self) -> Sf2DqsDoDlySelR {
        Sf2DqsDoDlySelR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf2_cs_dly_sel(&mut self) -> Sf2CsDlySelW<'_, Sf2IfIoDly0Spec> {
        Sf2CsDlySelW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sf2_cs2_dly_sel(&mut self) -> Sf2Cs2DlySelW<'_, Sf2IfIoDly0Spec> {
        Sf2Cs2DlySelW::new(self, 2)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf2_clk_out_dly_sel(&mut self) -> Sf2ClkOutDlySelW<'_, Sf2IfIoDly0Spec> {
        Sf2ClkOutDlySelW::new(self, 8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn sf2_dqs_oe_dly_sel(&mut self) -> Sf2DqsOeDlySelW<'_, Sf2IfIoDly0Spec> {
        Sf2DqsOeDlySelW::new(self, 26)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn sf2_dqs_di_dly_sel(&mut self) -> Sf2DqsDiDlySelW<'_, Sf2IfIoDly0Spec> {
        Sf2DqsDiDlySelW::new(self, 28)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn sf2_dqs_do_dly_sel(&mut self) -> Sf2DqsDoDlySelW<'_, Sf2IfIoDly0Spec> {
        Sf2DqsDoDlySelW::new(self, 30)
    }
}
#[doc = "SF2 IO Delay Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf2_if_io_dly_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf2_if_io_dly_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sf2IfIoDly0Spec;
impl crate::RegisterSpec for Sf2IfIoDly0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf2_if_io_dly_0::R`](R) reader structure"]
impl crate::Readable for Sf2IfIoDly0Spec {}
#[doc = "`write(|w| ..)` method takes [`sf2_if_io_dly_0::W`](W) writer structure"]
impl crate::Writable for Sf2IfIoDly0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF2_IF_IO_DLY_0 to value 0"]
impl crate::Resettable for Sf2IfIoDly0Spec {}
