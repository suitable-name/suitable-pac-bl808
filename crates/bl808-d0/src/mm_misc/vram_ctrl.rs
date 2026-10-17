#[doc = "Register `VRAM_CTRL` reader"]
pub type R = crate::R<VramCtrlSpec>;
#[doc = "Register `VRAM_CTRL` writer"]
pub type W = crate::W<VramCtrlSpec>;
#[doc = "Field `reg_sysram_set` writer - Trigger system RAM partition updates."]
pub type RegSysramSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_h2pf_sram_rel` reader - Release H2PF SRAM partitions."]
pub type RegH2pfSramRelR = crate::FieldReader;
#[doc = "Field `reg_h2pf_sram_rel` writer - Release H2PF SRAM partitions."]
pub type RegH2pfSramRelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `reg_vram_sram_rel` reader - Release VRAM SRAM partition."]
pub type RegVramSramRelR = crate::BitReader;
#[doc = "Field `reg_vram_sram_rel` writer - Release VRAM SRAM partition."]
pub type RegVramSramRelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_sub_sram_rel` reader - Release Subsystem SRAM partition."]
pub type RegSubSramRelR = crate::BitReader;
#[doc = "Field `reg_sub_sram_rel` writer - Release Subsystem SRAM partition."]
pub type RegSubSramRelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_blai_sram_rel` reader - Release BLAI (NPU) SRAM partition."]
pub type RegBlaiSramRelR = crate::BitReader;
#[doc = "Field `reg_blai_sram_rel` writer - Release BLAI (NPU) SRAM partition."]
pub type RegBlaiSramRelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_h2pf_sram_sel` reader - Current H2PF SRAM partition selection status."]
pub type RegH2pfSramSelR = crate::FieldReader;
#[doc = "Field `reg_vram_sram_sel` reader - Current VRAM SRAM partition selection status."]
pub type RegVramSramSelR = crate::BitReader;
#[doc = "Field `reg_sub_sram_sel` reader - Current Subsystem SRAM partition selection status."]
pub type RegSubSramSelR = crate::BitReader;
#[doc = "Field `reg_blai_sram_sel` reader - Current BLAI (NPU) SRAM partition selection status."]
pub type RegBlaiSramSelR = crate::BitReader;
impl R {
    #[doc = "Bits 1:2 - Release H2PF SRAM partitions."]
    #[inline(always)]
    pub fn reg_h2pf_sram_rel(&self) -> RegH2pfSramRelR {
        RegH2pfSramRelR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 4 - Release VRAM SRAM partition."]
    #[inline(always)]
    pub fn reg_vram_sram_rel(&self) -> RegVramSramRelR {
        RegVramSramRelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Release Subsystem SRAM partition."]
    #[inline(always)]
    pub fn reg_sub_sram_rel(&self) -> RegSubSramRelR {
        RegSubSramRelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Release BLAI (NPU) SRAM partition."]
    #[inline(always)]
    pub fn reg_blai_sram_rel(&self) -> RegBlaiSramRelR {
        RegBlaiSramRelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Current H2PF SRAM partition selection status."]
    #[inline(always)]
    pub fn reg_h2pf_sram_sel(&self) -> RegH2pfSramSelR {
        RegH2pfSramSelR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Current VRAM SRAM partition selection status."]
    #[inline(always)]
    pub fn reg_vram_sram_sel(&self) -> RegVramSramSelR {
        RegVramSramSelR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Current Subsystem SRAM partition selection status."]
    #[inline(always)]
    pub fn reg_sub_sram_sel(&self) -> RegSubSramSelR {
        RegSubSramSelR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Current BLAI (NPU) SRAM partition selection status."]
    #[inline(always)]
    pub fn reg_blai_sram_sel(&self) -> RegBlaiSramSelR {
        RegBlaiSramSelR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger system RAM partition updates."]
    #[inline(always)]
    pub fn reg_sysram_set(&mut self) -> RegSysramSetW<'_, VramCtrlSpec> {
        RegSysramSetW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Release H2PF SRAM partitions."]
    #[inline(always)]
    pub fn reg_h2pf_sram_rel(&mut self) -> RegH2pfSramRelW<'_, VramCtrlSpec> {
        RegH2pfSramRelW::new(self, 1)
    }
    #[doc = "Bit 4 - Release VRAM SRAM partition."]
    #[inline(always)]
    pub fn reg_vram_sram_rel(&mut self) -> RegVramSramRelW<'_, VramCtrlSpec> {
        RegVramSramRelW::new(self, 4)
    }
    #[doc = "Bit 6 - Release Subsystem SRAM partition."]
    #[inline(always)]
    pub fn reg_sub_sram_rel(&mut self) -> RegSubSramRelW<'_, VramCtrlSpec> {
        RegSubSramRelW::new(self, 6)
    }
    #[doc = "Bit 7 - Release BLAI (NPU) SRAM partition."]
    #[inline(always)]
    pub fn reg_blai_sram_rel(&mut self) -> RegBlaiSramRelW<'_, VramCtrlSpec> {
        RegBlaiSramRelW::new(self, 7)
    }
}
#[doc = "VRAM and Multimedia SRAM Partition Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vram_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vram_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VramCtrlSpec;
impl crate::RegisterSpec for VramCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vram_ctrl::R`](R) reader structure"]
impl crate::Readable for VramCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`vram_ctrl::W`](W) writer structure"]
impl crate::Writable for VramCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VRAM_CTRL to value 0"]
impl crate::Resettable for VramCtrlSpec {}
