#[doc = "Register `DVP2AXI_FRAM_EXM` reader"]
pub type R = crate::R<Dvp2axiFramExmSpec>;
#[doc = "Register `DVP2AXI_FRAM_EXM` writer"]
pub type W = crate::W<Dvp2axiFramExmSpec>;
#[doc = "Field `reg_total_hcnt` reader - Total HSYNC count"]
pub type RegTotalHcntR = crate::FieldReader<u16>;
#[doc = "Field `reg_total_hcnt` writer - Total HSYNC count"]
pub type RegTotalHcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `reg_total_vcnt` reader - Total VSYNC count"]
pub type RegTotalVcntR = crate::FieldReader<u16>;
#[doc = "Field `reg_total_vcnt` writer - Total VSYNC count"]
pub type RegTotalVcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Total HSYNC count"]
    #[inline(always)]
    pub fn reg_total_hcnt(&self) -> RegTotalHcntR {
        RegTotalHcntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Total VSYNC count"]
    #[inline(always)]
    pub fn reg_total_vcnt(&self) -> RegTotalVcntR {
        RegTotalVcntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Total HSYNC count"]
    #[inline(always)]
    pub fn reg_total_hcnt(&mut self) -> RegTotalHcntW<'_, Dvp2axiFramExmSpec> {
        RegTotalHcntW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Total VSYNC count"]
    #[inline(always)]
    pub fn reg_total_vcnt(&mut self) -> RegTotalVcntW<'_, Dvp2axiFramExmSpec> {
        RegTotalVcntW::new(self, 16)
    }
}
#[doc = "Frame Examination (Total H/V Count)\n\nYou can [`read`](crate::Reg::read) this register and get [`dvp2axi_fram_exm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvp2axi_fram_exm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dvp2axiFramExmSpec;
impl crate::RegisterSpec for Dvp2axiFramExmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvp2axi_fram_exm::R`](R) reader structure"]
impl crate::Readable for Dvp2axiFramExmSpec {}
#[doc = "`write(|w| ..)` method takes [`dvp2axi_fram_exm::W`](W) writer structure"]
impl crate::Writable for Dvp2axiFramExmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DVP2AXI_FRAM_EXM to value 0"]
impl crate::Resettable for Dvp2axiFramExmSpec {}
