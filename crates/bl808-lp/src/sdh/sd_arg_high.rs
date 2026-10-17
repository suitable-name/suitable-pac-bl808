#[doc = "Register `SD_ARG_HIGH` reader"]
pub type R = crate::R<SdArgHighSpec>;
#[doc = "Register `SD_ARG_HIGH` writer"]
pub type W = crate::W<SdArgHighSpec>;
#[doc = "Field `ARG_H` reader - "]
pub type ArgHR = crate::FieldReader<u16>;
#[doc = "Field `ARG_H` writer - "]
pub type ArgHW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn arg_h(&self) -> ArgHR {
        ArgHR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn arg_h(&mut self) -> ArgHW<'_, SdArgHighSpec> {
        ArgHW::new(self, 0)
    }
}
#[doc = "Command Argument High Register (Bits 31:16)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_arg_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_arg_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdArgHighSpec;
impl crate::RegisterSpec for SdArgHighSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_arg_high::R`](R) reader structure"]
impl crate::Readable for SdArgHighSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_arg_high::W`](W) writer structure"]
impl crate::Writable for SdArgHighSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_ARG_HIGH to value 0"]
impl crate::Resettable for SdArgHighSpec {}
