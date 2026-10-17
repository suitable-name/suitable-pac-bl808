#[doc = "Register `SD_ARG_LOW` reader"]
pub type R = crate::R<SdArgLowSpec>;
#[doc = "Register `SD_ARG_LOW` writer"]
pub type W = crate::W<SdArgLowSpec>;
#[doc = "Field `ARG_L` reader - "]
pub type ArgLR = crate::FieldReader<u16>;
#[doc = "Field `ARG_L` writer - "]
pub type ArgLW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn arg_l(&self) -> ArgLR {
        ArgLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn arg_l(&mut self) -> ArgLW<'_, SdArgLowSpec> {
        ArgLW::new(self, 0)
    }
}
#[doc = "Command Argument Low Register (Bits 15:0)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_arg_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_arg_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdArgLowSpec;
impl crate::RegisterSpec for SdArgLowSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_arg_low::R`](R) reader structure"]
impl crate::Readable for SdArgLowSpec {}
#[doc = "`write(|w| ..)` method takes [`sd_arg_low::W`](W) writer structure"]
impl crate::Writable for SdArgLowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_ARG_LOW to value 0"]
impl crate::Resettable for SdArgLowSpec {}
