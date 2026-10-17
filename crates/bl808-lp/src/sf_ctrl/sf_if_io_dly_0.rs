#[doc = "Register `SF_IF_IO_DLY_0` reader"]
pub type R = crate::R<SfIfIoDly0Spec>;
#[doc = "Register `SF_IF_IO_DLY_0` writer"]
pub type W = crate::W<SfIfIoDly0Spec>;
#[doc = "Field `sf_cs_dly_sel` reader - "]
pub type SfCsDlySelR = crate::FieldReader;
#[doc = "Field `sf_cs_dly_sel` writer - "]
pub type SfCsDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf_cs2_dly_sel` reader - "]
pub type SfCs2DlySelR = crate::FieldReader;
#[doc = "Field `sf_cs2_dly_sel` writer - "]
pub type SfCs2DlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf_clk_out_dly_sel` reader - "]
pub type SfClkOutDlySelR = crate::FieldReader;
#[doc = "Field `sf_clk_out_dly_sel` writer - "]
pub type SfClkOutDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf_dqs_oe_dly_sel` reader - "]
pub type SfDqsOeDlySelR = crate::FieldReader;
#[doc = "Field `sf_dqs_oe_dly_sel` writer - "]
pub type SfDqsOeDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf_dqs_di_dly_sel` reader - "]
pub type SfDqsDiDlySelR = crate::FieldReader;
#[doc = "Field `sf_dqs_di_dly_sel` writer - "]
pub type SfDqsDiDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sf_dqs_do_dly_sel` reader - "]
pub type SfDqsDoDlySelR = crate::FieldReader;
#[doc = "Field `sf_dqs_do_dly_sel` writer - "]
pub type SfDqsDoDlySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf_cs_dly_sel(&self) -> SfCsDlySelR {
        SfCsDlySelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sf_cs2_dly_sel(&self) -> SfCs2DlySelR {
        SfCs2DlySelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf_clk_out_dly_sel(&self) -> SfClkOutDlySelR {
        SfClkOutDlySelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn sf_dqs_oe_dly_sel(&self) -> SfDqsOeDlySelR {
        SfDqsOeDlySelR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn sf_dqs_di_dly_sel(&self) -> SfDqsDiDlySelR {
        SfDqsDiDlySelR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn sf_dqs_do_dly_sel(&self) -> SfDqsDoDlySelR {
        SfDqsDoDlySelR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf_cs_dly_sel(&mut self) -> SfCsDlySelW<'_, SfIfIoDly0Spec> {
        SfCsDlySelW::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sf_cs2_dly_sel(&mut self) -> SfCs2DlySelW<'_, SfIfIoDly0Spec> {
        SfCs2DlySelW::new(self, 2)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf_clk_out_dly_sel(&mut self) -> SfClkOutDlySelW<'_, SfIfIoDly0Spec> {
        SfClkOutDlySelW::new(self, 8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn sf_dqs_oe_dly_sel(&mut self) -> SfDqsOeDlySelW<'_, SfIfIoDly0Spec> {
        SfDqsOeDlySelW::new(self, 26)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn sf_dqs_di_dly_sel(&mut self) -> SfDqsDiDlySelW<'_, SfIfIoDly0Spec> {
        SfDqsDiDlySelW::new(self, 28)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn sf_dqs_do_dly_sel(&mut self) -> SfDqsDoDlySelW<'_, SfIfIoDly0Spec> {
        SfDqsDoDlySelW::new(self, 30)
    }
}
#[doc = "SF IO Delay Configuration 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sf_if_io_dly_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sf_if_io_dly_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfIfIoDly0Spec;
impl crate::RegisterSpec for SfIfIoDly0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sf_if_io_dly_0::R`](R) reader structure"]
impl crate::Readable for SfIfIoDly0Spec {}
#[doc = "`write(|w| ..)` method takes [`sf_if_io_dly_0::W`](W) writer structure"]
impl crate::Writable for SfIfIoDly0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SF_IF_IO_DLY_0 to value 0"]
impl crate::Resettable for SfIfIoDly0Spec {}
