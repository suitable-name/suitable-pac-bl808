#[doc = "Register `APU_DM0_ADDR` reader"]
pub type R = crate::R<ApuDm0AddrSpec>;
#[doc = "Register `APU_DM0_ADDR` writer"]
pub type W = crate::W<ApuDm0AddrSpec>;
#[doc = "Field `reg_img_addr_base` reader - Base address for image data (inputs/outputs)."]
pub type RegImgAddrBaseR = crate::FieldReader<u32>;
#[doc = "Field `reg_img_addr_base` writer - Base address for image data (inputs/outputs)."]
pub type RegImgAddrBaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Base address for image data (inputs/outputs)."]
    #[inline(always)]
    pub fn reg_img_addr_base(&self) -> RegImgAddrBaseR {
        RegImgAddrBaseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address for image data (inputs/outputs)."]
    #[inline(always)]
    pub fn reg_img_addr_base(&mut self) -> RegImgAddrBaseW<'_, ApuDm0AddrSpec> {
        RegImgAddrBaseW::new(self, 0)
    }
}
#[doc = "Image Data (Input/Output) Base Address Pointer (DM0)\n\nYou can [`read`](crate::Reg::read) this register and get [`apu_dm0_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apu_dm0_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApuDm0AddrSpec;
impl crate::RegisterSpec for ApuDm0AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apu_dm0_addr::R`](R) reader structure"]
impl crate::Readable for ApuDm0AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`apu_dm0_addr::W`](W) writer structure"]
impl crate::Writable for ApuDm0AddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APU_DM0_ADDR to value 0"]
impl crate::Resettable for ApuDm0AddrSpec {}
