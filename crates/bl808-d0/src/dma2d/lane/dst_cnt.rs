#[doc = "Register `DST_CNT` reader"]
pub type R = crate::R<DstCntSpec>;
#[doc = "Register `DST_CNT` writer"]
pub type W = crate::W<DstCntSpec>;
#[doc = "Field `DST_X_CNT` reader - "]
pub type DstXCntR = crate::FieldReader<u16>;
#[doc = "Field `DST_X_CNT` writer - "]
pub type DstXCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dst_x_cnt(&self) -> DstXCntR {
        DstXCntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dst_x_cnt(&mut self) -> DstXCntW<'_, DstCntSpec> {
        DstXCntW::new(self, 0)
    }
}
#[doc = "Destination 2D Width\n\nYou can [`read`](crate::Reg::read) this register and get [`dst_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstCntSpec;
impl crate::RegisterSpec for DstCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst_cnt::R`](R) reader structure"]
impl crate::Readable for DstCntSpec {}
#[doc = "`write(|w| ..)` method takes [`dst_cnt::W`](W) writer structure"]
impl crate::Writable for DstCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DST_CNT to value 0"]
impl crate::Resettable for DstCntSpec {}
