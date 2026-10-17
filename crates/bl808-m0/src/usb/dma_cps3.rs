#[doc = "Register `DMA_CPS3` reader"]
pub type R = crate::R<DmaCps3Spec>;
#[doc = "Register `DMA_CPS3` writer"]
pub type W = crate::W<DmaCps3Spec>;
#[doc = "Field `SETUP_CMD_RPORT` reader - Setup Command / Read Port"]
pub type SetupCmdRportR = crate::FieldReader<u32>;
#[doc = "Field `SETUP_CMD_RPORT` writer - Setup Command / Read Port"]
pub type SetupCmdRportW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Setup Command / Read Port"]
    #[inline(always)]
    pub fn setup_cmd_rport(&self) -> SetupCmdRportR {
        SetupCmdRportR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Setup Command / Read Port"]
    #[inline(always)]
    pub fn setup_cmd_rport(&mut self) -> SetupCmdRportW<'_, DmaCps3Spec> {
        SetupCmdRportW::new(self, 0)
    }
}
#[doc = "DMA Control Packet Set 3 (Setup/Command)\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_cps3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_cps3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaCps3Spec;
impl crate::RegisterSpec for DmaCps3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_cps3::R`](R) reader structure"]
impl crate::Readable for DmaCps3Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_cps3::W`](W) writer structure"]
impl crate::Writable for DmaCps3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CPS3 to value 0"]
impl crate::Resettable for DmaCps3Spec {}
