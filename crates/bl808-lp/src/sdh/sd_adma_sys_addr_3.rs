#[doc = "Register `SD_ADMA_SYS_ADDR_3` reader"]
pub type R = crate::R<SdAdmaSysAddr3Spec>;
#[doc = "Register `SD_ADMA_SYS_ADDR_3` writer"]
pub type W = crate::W<SdAdmaSysAddr3Spec>;
#[doc = "Field `ADMA_SYS_ADDR` reader - "]
pub type AdmaSysAddrR = crate::FieldReader<u16>;
#[doc = "Field `ADMA_SYS_ADDR` writer - "]
pub type AdmaSysAddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn adma_sys_addr(&self) -> AdmaSysAddrR {
        AdmaSysAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn adma_sys_addr(&mut self) -> AdmaSysAddrW<'_, SdAdmaSysAddr3Spec> {
        AdmaSysAddrW::new(self, 0)
    }
}
#[doc = "ADMA System Address Register 3 (Bits 47:32)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_adma_sys_addr_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_adma_sys_addr_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdAdmaSysAddr3Spec;
impl crate::RegisterSpec for SdAdmaSysAddr3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_adma_sys_addr_3::R`](R) reader structure"]
impl crate::Readable for SdAdmaSysAddr3Spec {}
#[doc = "`write(|w| ..)` method takes [`sd_adma_sys_addr_3::W`](W) writer structure"]
impl crate::Writable for SdAdmaSysAddr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_ADMA_SYS_ADDR_3 to value 0"]
impl crate::Resettable for SdAdmaSysAddr3Spec {}
