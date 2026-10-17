#[doc = "Register `SD_BUFFER_DATA_PORT_1` reader"]
pub type R = crate::R<SdBufferDataPort1Spec>;
#[doc = "Register `SD_BUFFER_DATA_PORT_1` writer"]
pub type W = crate::W<SdBufferDataPort1Spec>;
#[doc = "Field `CPU_DATA1` reader - "]
pub type CpuData1R = crate::FieldReader<u16>;
#[doc = "Field `CPU_DATA1` writer - "]
pub type CpuData1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cpu_data1(&self) -> CpuData1R {
        CpuData1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cpu_data1(&mut self) -> CpuData1W<'_, SdBufferDataPort1Spec> {
        CpuData1W::new(self, 0)
    }
}
#[doc = "Buffer Data Port 1 (16 bits)\n\nYou can [`read`](crate::Reg::read) this register and get [`sd_buffer_data_port_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sd_buffer_data_port_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdBufferDataPort1Spec;
impl crate::RegisterSpec for SdBufferDataPort1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sd_buffer_data_port_1::R`](R) reader structure"]
impl crate::Readable for SdBufferDataPort1Spec {}
#[doc = "`write(|w| ..)` method takes [`sd_buffer_data_port_1::W`](W) writer structure"]
impl crate::Writable for SdBufferDataPort1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SD_BUFFER_DATA_PORT_1 to value 0"]
impl crate::Resettable for SdBufferDataPort1Spec {}
