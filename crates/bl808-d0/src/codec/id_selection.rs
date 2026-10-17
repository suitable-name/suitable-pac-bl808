#[doc = "Register `ID_SELECTION` reader"]
pub type R = crate::R<IdSelectionSpec>;
#[doc = "Register `ID_SELECTION` writer"]
pub type W = crate::W<IdSelectionSpec>;
#[doc = "Field `RG_JENC_ID_SEL` reader - MJPEG Encoder ID Selection"]
pub type RgJencIdSelR = crate::BitReader;
#[doc = "Field `RG_JENC_ID_SEL` writer - MJPEG Encoder ID Selection"]
pub type RgJencIdSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MJPEG Encoder ID Selection"]
    #[inline(always)]
    pub fn rg_jenc_id_sel(&self) -> RgJencIdSelR {
        RgJencIdSelR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MJPEG Encoder ID Selection"]
    #[inline(always)]
    pub fn rg_jenc_id_sel(&mut self) -> RgJencIdSelW<'_, IdSelectionSpec> {
        RgJencIdSelW::new(self, 0)
    }
}
#[doc = "ID Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`id_selection::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`id_selection::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdSelectionSpec;
impl crate::RegisterSpec for IdSelectionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id_selection::R`](R) reader structure"]
impl crate::Readable for IdSelectionSpec {}
#[doc = "`write(|w| ..)` method takes [`id_selection::W`](W) writer structure"]
impl crate::Writable for IdSelectionSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ID_SELECTION to value 0"]
impl crate::Resettable for IdSelectionSpec {}
