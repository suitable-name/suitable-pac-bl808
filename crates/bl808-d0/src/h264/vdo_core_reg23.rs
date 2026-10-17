#[doc = "Register `VDO_CORE_REG23` reader"]
pub type R = crate::R<VdoCoreReg23Spec>;
#[doc = "Register `VDO_CORE_REG23` writer"]
pub type W = crate::W<VdoCoreReg23Spec>;
#[doc = "Field `num_imb_bits` reader - Main-stream target I-MB bit count (CBR)"]
pub type NumImbBitsR = crate::FieldReader<u16>;
#[doc = "Field `num_imb_bits` writer - Main-stream target I-MB bit count (CBR)"]
pub type NumImbBitsW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `nal_align` reader - Main-stream NAL alignment enable"]
pub type NalAlignR = crate::BitReader;
#[doc = "Field `nal_align` writer - Main-stream NAL alignment enable"]
pub type NalAlignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cons_ipred` reader - Main-stream constrained intra prediction"]
pub type ConsIpredR = crate::BitReader;
#[doc = "Field `cons_ipred` writer - Main-stream constrained intra prediction"]
pub type ConsIpredW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `s_num_imb_bits` reader - Secondary-stream target I-MB bit count (CBR)"]
pub type SNumImbBitsR = crate::FieldReader<u16>;
#[doc = "Field `s_num_imb_bits` writer - Secondary-stream target I-MB bit count (CBR)"]
pub type SNumImbBitsW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `s_nal_align` reader - Secondary-stream NAL alignment enable"]
pub type SNalAlignR = crate::BitReader;
#[doc = "Field `s_nal_align` writer - Secondary-stream NAL alignment enable"]
pub type SNalAlignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `s_cons_ipred` reader - Secondary-stream constrained intra prediction"]
pub type SConsIpredR = crate::BitReader;
#[doc = "Field `s_cons_ipred` writer - Secondary-stream constrained intra prediction"]
pub type SConsIpredW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Main-stream target I-MB bit count (CBR)"]
    #[inline(always)]
    pub fn num_imb_bits(&self) -> NumImbBitsR {
        NumImbBitsR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 13 - Main-stream NAL alignment enable"]
    #[inline(always)]
    pub fn nal_align(&self) -> NalAlignR {
        NalAlignR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Main-stream constrained intra prediction"]
    #[inline(always)]
    pub fn cons_ipred(&self) -> ConsIpredR {
        ConsIpredR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:27 - Secondary-stream target I-MB bit count (CBR)"]
    #[inline(always)]
    pub fn s_num_imb_bits(&self) -> SNumImbBitsR {
        SNumImbBitsR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 29 - Secondary-stream NAL alignment enable"]
    #[inline(always)]
    pub fn s_nal_align(&self) -> SNalAlignR {
        SNalAlignR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Secondary-stream constrained intra prediction"]
    #[inline(always)]
    pub fn s_cons_ipred(&self) -> SConsIpredR {
        SConsIpredR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Main-stream target I-MB bit count (CBR)"]
    #[inline(always)]
    pub fn num_imb_bits(&mut self) -> NumImbBitsW<'_, VdoCoreReg23Spec> {
        NumImbBitsW::new(self, 0)
    }
    #[doc = "Bit 13 - Main-stream NAL alignment enable"]
    #[inline(always)]
    pub fn nal_align(&mut self) -> NalAlignW<'_, VdoCoreReg23Spec> {
        NalAlignW::new(self, 13)
    }
    #[doc = "Bit 14 - Main-stream constrained intra prediction"]
    #[inline(always)]
    pub fn cons_ipred(&mut self) -> ConsIpredW<'_, VdoCoreReg23Spec> {
        ConsIpredW::new(self, 14)
    }
    #[doc = "Bits 16:27 - Secondary-stream target I-MB bit count (CBR)"]
    #[inline(always)]
    pub fn s_num_imb_bits(&mut self) -> SNumImbBitsW<'_, VdoCoreReg23Spec> {
        SNumImbBitsW::new(self, 16)
    }
    #[doc = "Bit 29 - Secondary-stream NAL alignment enable"]
    #[inline(always)]
    pub fn s_nal_align(&mut self) -> SNalAlignW<'_, VdoCoreReg23Spec> {
        SNalAlignW::new(self, 29)
    }
    #[doc = "Bit 30 - Secondary-stream constrained intra prediction"]
    #[inline(always)]
    pub fn s_cons_ipred(&mut self) -> SConsIpredW<'_, VdoCoreReg23Spec> {
        SConsIpredW::new(self, 30)
    }
}
#[doc = "I-frame macroblock bit count and NAL/prediction options\n\nYou can [`read`](crate::Reg::read) this register and get [`vdo_core_reg23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdo_core_reg23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VdoCoreReg23Spec;
impl crate::RegisterSpec for VdoCoreReg23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdo_core_reg23::R`](R) reader structure"]
impl crate::Readable for VdoCoreReg23Spec {}
#[doc = "`write(|w| ..)` method takes [`vdo_core_reg23::W`](W) writer structure"]
impl crate::Writable for VdoCoreReg23Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VDO_CORE_REG23 to value 0"]
impl crate::Resettable for VdoCoreReg23Spec {}
