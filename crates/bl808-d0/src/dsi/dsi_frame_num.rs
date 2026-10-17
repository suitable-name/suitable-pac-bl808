#[doc = "Register `DSI_FRAME_NUM` reader"]
pub type R = crate::R<DsiFrameNumSpec>;
#[doc = "Register `DSI_FRAME_NUM` writer"]
pub type W = crate::W<DsiFrameNumSpec>;
#[doc = "Field `cr_frame_num` reader - "]
pub type CrFrameNumR = crate::FieldReader<u16>;
#[doc = "Field `cr_frame_num` writer - "]
pub type CrFrameNumW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `cr_frame_upd` writer - Frame number update pulse"]
pub type CrFrameUpdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cr_frame_num(&self) -> CrFrameNumR {
        CrFrameNumR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cr_frame_num(&mut self) -> CrFrameNumW<'_, DsiFrameNumSpec> {
        CrFrameNumW::new(self, 0)
    }
    #[doc = "Bit 31 - Frame number update pulse"]
    #[inline(always)]
    pub fn cr_frame_upd(&mut self) -> CrFrameUpdW<'_, DsiFrameNumSpec> {
        CrFrameUpdW::new(self, 31)
    }
}
#[doc = "DSI Frame Number Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_frame_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_frame_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiFrameNumSpec;
impl crate::RegisterSpec for DsiFrameNumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_frame_num::R`](R) reader structure"]
impl crate::Readable for DsiFrameNumSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_frame_num::W`](W) writer structure"]
impl crate::Writable for DsiFrameNumSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_FRAME_NUM to value 0"]
impl crate::Resettable for DsiFrameNumSpec {}
