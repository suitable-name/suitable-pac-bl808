#[doc = "Register `SRC_CNT` reader"]
pub type R = crate::R<SrcCntSpec>;
#[doc = "Register `SRC_CNT` writer"]
pub type W = crate::W<SrcCntSpec>;
#[doc = "Field `SRC_X_CNT` reader - Source width in bytes/words"]
pub type SrcXCntR = crate::FieldReader<u16>;
#[doc = "Field `SRC_X_CNT` writer - Source width in bytes/words"]
pub type SrcXCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SRC_Y_CNT` reader - Source height in lines"]
pub type SrcYCntR = crate::FieldReader<u16>;
#[doc = "Field `SRC_Y_CNT` writer - Source height in lines"]
pub type SrcYCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Source width in bytes/words"]
    #[inline(always)]
    pub fn src_x_cnt(&self) -> SrcXCntR {
        SrcXCntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Source height in lines"]
    #[inline(always)]
    pub fn src_y_cnt(&self) -> SrcYCntR {
        SrcYCntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Source width in bytes/words"]
    #[inline(always)]
    pub fn src_x_cnt(&mut self) -> SrcXCntW<'_, SrcCntSpec> {
        SrcXCntW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Source height in lines"]
    #[inline(always)]
    pub fn src_y_cnt(&mut self) -> SrcYCntW<'_, SrcCntSpec> {
        SrcYCntW::new(self, 16)
    }
}
#[doc = "Source 2D Dimensions\n\nYou can [`read`](crate::Reg::read) this register and get [`src_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcCntSpec;
impl crate::RegisterSpec for SrcCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src_cnt::R`](R) reader structure"]
impl crate::Readable for SrcCntSpec {}
#[doc = "`write(|w| ..)` method takes [`src_cnt::W`](W) writer structure"]
impl crate::Writable for SrcCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRC_CNT to value 0"]
impl crate::Resettable for SrcCntSpec {}
