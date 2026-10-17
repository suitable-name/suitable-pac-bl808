#[doc = "Register `VDO_H264_S_REFU1_BASE_ADDR` reader"]
pub type R = crate::R<VdoH264SRefu1BaseAddrSpec>;
#[doc = "Register `VDO_H264_S_REFU1_BASE_ADDR` writer"]
pub type W = crate::W<VdoH264SRefu1BaseAddrSpec>;
#[doc = "Field `cfg_s_refu1_base_addr` reader - "]
pub type CfgSRefu1BaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `cfg_s_refu1_base_addr` writer - "]
pub type CfgSRefu1BaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cfg_s_refu1_base_addr(&self) -> CfgSRefu1BaseAddrR {
        CfgSRefu1BaseAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cfg_s_refu1_base_addr(&mut self) -> CfgSRefu1BaseAddrW<'_, VdoH264SRefu1BaseAddrSpec> {
        CfgSRefu1BaseAddrW::new(self, 0)
    }
}
#[doc = "Secondary-stream reference frame 1 U base address\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_h264_s_refu1_base_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_h264_s_refu1_base_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdoH264SRefu1BaseAddrSpec;
impl crate::RegisterSpec for VdoH264SRefu1BaseAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdo_h264_s_refu1_base_addr::R`](R) reader structure"]
impl crate::Readable for VdoH264SRefu1BaseAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`vdo_h264_s_refu1_base_addr::W`](W) writer structure"]
impl crate::Writable for VdoH264SRefu1BaseAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDO_H264_S_REFU1_BASE_ADDR to value 0"]
impl crate::Resettable for VdoH264SRefu1BaseAddrSpec {}
