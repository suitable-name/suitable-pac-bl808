#[doc = "Register `DLL_CFG0` reader"]
pub type R = crate::R<DllCfg0Spec>;
#[doc = "Register `DLL_CFG0` writer"]
pub type W = crate::W<DllCfg0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DLL Configuration Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dll_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dll_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DllCfg0Spec;
impl crate::RegisterSpec for DllCfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dll_cfg0::R`](R) reader structure"]
impl crate::Readable for DllCfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`dll_cfg0::W`](W) writer structure"]
impl crate::Writable for DllCfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DLL_CFG0 to value 0"]
impl crate::Resettable for DllCfg0Spec {}
