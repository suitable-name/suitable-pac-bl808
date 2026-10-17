#[doc = "Register `VDMA_CTRL` reader"]
pub type R = crate::R<VdmaCtrlSpec>;
#[doc = "Register `VDMA_CTRL` writer"]
pub type W = crate::W<VdmaCtrlSpec>;
#[doc = "Field `VDMA_EN` reader - "]
pub type VdmaEnR = crate::BitReader;
#[doc = "Field `VDMA_EN` writer - "]
pub type VdmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vdma_en(&self) -> VdmaEnR {
        VdmaEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vdma_en(&mut self) -> VdmaEnW<'_, VdmaCtrlSpec> {
        VdmaEnW::new(self, 0)
    }
}
#[doc = "VDMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vdma_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdma_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdmaCtrlSpec;
impl crate::RegisterSpec for VdmaCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdma_ctrl::R`](R) reader structure"]
impl crate::Readable for VdmaCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`vdma_ctrl::W`](W) writer structure"]
impl crate::Writable for VdmaCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDMA_CTRL to value 0"]
impl crate::Resettable for VdmaCtrlSpec {}
