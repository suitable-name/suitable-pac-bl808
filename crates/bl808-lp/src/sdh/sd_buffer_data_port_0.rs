#[doc = "Register `SD_BUFFER_DATA_PORT_0` reader"]
pub type R = crate::R<SdBufferDataPort0Spec>;
#[doc = "Register `SD_BUFFER_DATA_PORT_0` writer"]
pub type W = crate::W<SdBufferDataPort0Spec>;
#[doc = "Field `CPU_DATA0` reader - "]
pub type CpuData0R = crate::FieldReader<u16>;
#[doc = "Field `CPU_DATA0` writer - "]
pub type CpuData0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cpu_data0(&self) -> CpuData0R {
        CpuData0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cpu_data0(&mut self) -> CpuData0W<'_, SdBufferDataPort0Spec> {
        CpuData0W::new(self, 0)
    }
}
#[doc = "Buffer Data Port 0 (16 bits)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_buffer_data_port_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_buffer_data_port_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdBufferDataPort0Spec;
impl crate::RegisterSpec for SdBufferDataPort0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_buffer_data_port_0::R`](R) reader structure"]
impl crate::Readable for SdBufferDataPort0Spec {}
#[doc = "`write(|w| ..)` method takes [`sd_buffer_data_port_0::W`](W) writer structure"]
impl crate::Writable for SdBufferDataPort0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_BUFFER_DATA_PORT_0 to value 0"]
impl crate::Resettable for SdBufferDataPort0Spec {}
