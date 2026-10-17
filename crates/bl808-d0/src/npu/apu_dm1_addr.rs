#[doc = "Register `APU_DM1_ADDR` reader"]
pub type R = crate::R<ApuDm1AddrSpec>;
#[doc = "Register `APU_DM1_ADDR` writer"]
pub type W = crate::W<ApuDm1AddrSpec>;
#[doc = "Field `reg_img_mem_seg` reader - Image memory segmentation configuration"]
pub type RegImgMemSegR = crate::FieldReader<u32>;
#[doc = "Field `reg_img_mem_seg` writer - Image memory segmentation configuration"]
pub type RegImgMemSegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Image memory segmentation configuration"]
    #[inline(always)]
    pub fn reg_img_mem_seg(&self) -> RegImgMemSegR {
        RegImgMemSegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Image memory segmentation configuration"]
    #[inline(always)]
    pub fn reg_img_mem_seg(&mut self) -> RegImgMemSegW<'_, ApuDm1AddrSpec> {
        RegImgMemSegW::new(self, 0)
    }
}
#[doc = "Image Memory Segment Configuration (DM1)\n\nYou can [`read`](crate::Reg::read) this register and get [`apu_dm1_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apu_dm1_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApuDm1AddrSpec;
impl crate::RegisterSpec for ApuDm1AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apu_dm1_addr::R`](R) reader structure"]
impl crate::Readable for ApuDm1AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`apu_dm1_addr::W`](W) writer structure"]
impl crate::Writable for ApuDm1AddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APU_DM1_ADDR to value 0"]
impl crate::Resettable for ApuDm1AddrSpec {}
