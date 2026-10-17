#[doc = "Register `SRAM_PARM` reader"]
pub type R = crate::R<SramParmSpec>;
#[doc = "Register `SRAM_PARM` writer"]
pub type W = crate::W<SramParmSpec>;
#[doc = "Field `reg_sram_cpu_ram_dvs` reader - CPU RAM DVS target."]
pub type RegSramCpuRamDvsR = crate::FieldReader;
#[doc = "Field `reg_sram_cpu_ram_dvs` writer - CPU RAM DVS target."]
pub type RegSramCpuRamDvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_sram_cpu_ram_dvse` reader - CPU RAM DVS enable."]
pub type RegSramCpuRamDvseR = crate::BitReader;
#[doc = "Field `reg_sram_cpu_ram_dvse` writer - CPU RAM DVS enable."]
pub type RegSramCpuRamDvseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_sram_cpu_ram_nap` reader - CPU RAM Nap mode enable."]
pub type RegSramCpuRamNapR = crate::BitReader;
#[doc = "Field `reg_sram_cpu_ram_nap` writer - CPU RAM Nap mode enable."]
pub type RegSramCpuRamNapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_sram_l2ram_dvs` reader - L2 RAM DVS target."]
pub type RegSramL2ramDvsR = crate::FieldReader;
#[doc = "Field `reg_sram_l2ram_dvs` writer - L2 RAM DVS target."]
pub type RegSramL2ramDvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_sram_l2ram_dvse` reader - L2 RAM DVS enable."]
pub type RegSramL2ramDvseR = crate::BitReader;
#[doc = "Field `reg_sram_l2ram_dvse` writer - L2 RAM DVS enable."]
pub type RegSramL2ramDvseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_sram_l2ram_nap` reader - L2 RAM Nap mode enable."]
pub type RegSramL2ramNapR = crate::BitReader;
#[doc = "Field `reg_sram_l2ram_nap` writer - L2 RAM Nap mode enable."]
pub type RegSramL2ramNapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_sram_cdc_ram_dvs` reader - Codec RAM DVS target."]
pub type RegSramCdcRamDvsR = crate::FieldReader;
#[doc = "Field `reg_sram_cdc_ram_dvs` writer - Codec RAM DVS target."]
pub type RegSramCdcRamDvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_sram_cdc_ram_dvse` reader - Codec RAM DVS enable."]
pub type RegSramCdcRamDvseR = crate::BitReader;
#[doc = "Field `reg_sram_cdc_ram_dvse` writer - Codec RAM DVS enable."]
pub type RegSramCdcRamDvseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_sram_cdc_ram_nap` reader - Codec RAM Nap mode enable."]
pub type RegSramCdcRamNapR = crate::BitReader;
#[doc = "Field `reg_sram_cdc_ram_nap` writer - Codec RAM Nap mode enable."]
pub type RegSramCdcRamNapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_sram_sub_ram_dvs` reader - Subsystem RAM DVS target."]
pub type RegSramSubRamDvsR = crate::FieldReader;
#[doc = "Field `reg_sram_sub_ram_dvs` writer - Subsystem RAM DVS target."]
pub type RegSramSubRamDvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_sram_sub_ram_dvse` reader - Subsystem RAM DVS enable."]
pub type RegSramSubRamDvseR = crate::BitReader;
#[doc = "Field `reg_sram_sub_ram_dvse` writer - Subsystem RAM DVS enable."]
pub type RegSramSubRamDvseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_sram_sub_ram_nap` reader - Subsystem RAM Nap mode enable."]
pub type RegSramSubRamNapR = crate::BitReader;
#[doc = "Field `reg_sram_sub_ram_nap` writer - Subsystem RAM Nap mode enable."]
pub type RegSramSubRamNapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - CPU RAM DVS target."]
    #[inline(always)]
    pub fn reg_sram_cpu_ram_dvs(&self) -> RegSramCpuRamDvsR {
        RegSramCpuRamDvsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - CPU RAM DVS enable."]
    #[inline(always)]
    pub fn reg_sram_cpu_ram_dvse(&self) -> RegSramCpuRamDvseR {
        RegSramCpuRamDvseR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU RAM Nap mode enable."]
    #[inline(always)]
    pub fn reg_sram_cpu_ram_nap(&self) -> RegSramCpuRamNapR {
        RegSramCpuRamNapR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - L2 RAM DVS target."]
    #[inline(always)]
    pub fn reg_sram_l2ram_dvs(&self) -> RegSramL2ramDvsR {
        RegSramL2ramDvsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - L2 RAM DVS enable."]
    #[inline(always)]
    pub fn reg_sram_l2ram_dvse(&self) -> RegSramL2ramDvseR {
        RegSramL2ramDvseR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - L2 RAM Nap mode enable."]
    #[inline(always)]
    pub fn reg_sram_l2ram_nap(&self) -> RegSramL2ramNapR {
        RegSramL2ramNapR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Codec RAM DVS target."]
    #[inline(always)]
    pub fn reg_sram_cdc_ram_dvs(&self) -> RegSramCdcRamDvsR {
        RegSramCdcRamDvsR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Codec RAM DVS enable."]
    #[inline(always)]
    pub fn reg_sram_cdc_ram_dvse(&self) -> RegSramCdcRamDvseR {
        RegSramCdcRamDvseR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Codec RAM Nap mode enable."]
    #[inline(always)]
    pub fn reg_sram_cdc_ram_nap(&self) -> RegSramCdcRamNapR {
        RegSramCdcRamNapR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Subsystem RAM DVS target."]
    #[inline(always)]
    pub fn reg_sram_sub_ram_dvs(&self) -> RegSramSubRamDvsR {
        RegSramSubRamDvsR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Subsystem RAM DVS enable."]
    #[inline(always)]
    pub fn reg_sram_sub_ram_dvse(&self) -> RegSramSubRamDvseR {
        RegSramSubRamDvseR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Subsystem RAM Nap mode enable."]
    #[inline(always)]
    pub fn reg_sram_sub_ram_nap(&self) -> RegSramSubRamNapR {
        RegSramSubRamNapR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - CPU RAM DVS target."]
    #[inline(always)]
    pub fn reg_sram_cpu_ram_dvs(&mut self) -> RegSramCpuRamDvsW<'_, SramParmSpec> {
        RegSramCpuRamDvsW::new(self, 0)
    }
    #[doc = "Bit 4 - CPU RAM DVS enable."]
    #[inline(always)]
    pub fn reg_sram_cpu_ram_dvse(&mut self) -> RegSramCpuRamDvseW<'_, SramParmSpec> {
        RegSramCpuRamDvseW::new(self, 4)
    }
    #[doc = "Bit 5 - CPU RAM Nap mode enable."]
    #[inline(always)]
    pub fn reg_sram_cpu_ram_nap(&mut self) -> RegSramCpuRamNapW<'_, SramParmSpec> {
        RegSramCpuRamNapW::new(self, 5)
    }
    #[doc = "Bits 8:11 - L2 RAM DVS target."]
    #[inline(always)]
    pub fn reg_sram_l2ram_dvs(&mut self) -> RegSramL2ramDvsW<'_, SramParmSpec> {
        RegSramL2ramDvsW::new(self, 8)
    }
    #[doc = "Bit 12 - L2 RAM DVS enable."]
    #[inline(always)]
    pub fn reg_sram_l2ram_dvse(&mut self) -> RegSramL2ramDvseW<'_, SramParmSpec> {
        RegSramL2ramDvseW::new(self, 12)
    }
    #[doc = "Bit 13 - L2 RAM Nap mode enable."]
    #[inline(always)]
    pub fn reg_sram_l2ram_nap(&mut self) -> RegSramL2ramNapW<'_, SramParmSpec> {
        RegSramL2ramNapW::new(self, 13)
    }
    #[doc = "Bits 16:19 - Codec RAM DVS target."]
    #[inline(always)]
    pub fn reg_sram_cdc_ram_dvs(&mut self) -> RegSramCdcRamDvsW<'_, SramParmSpec> {
        RegSramCdcRamDvsW::new(self, 16)
    }
    #[doc = "Bit 20 - Codec RAM DVS enable."]
    #[inline(always)]
    pub fn reg_sram_cdc_ram_dvse(&mut self) -> RegSramCdcRamDvseW<'_, SramParmSpec> {
        RegSramCdcRamDvseW::new(self, 20)
    }
    #[doc = "Bit 21 - Codec RAM Nap mode enable."]
    #[inline(always)]
    pub fn reg_sram_cdc_ram_nap(&mut self) -> RegSramCdcRamNapW<'_, SramParmSpec> {
        RegSramCdcRamNapW::new(self, 21)
    }
    #[doc = "Bits 24:27 - Subsystem RAM DVS target."]
    #[inline(always)]
    pub fn reg_sram_sub_ram_dvs(&mut self) -> RegSramSubRamDvsW<'_, SramParmSpec> {
        RegSramSubRamDvsW::new(self, 24)
    }
    #[doc = "Bit 28 - Subsystem RAM DVS enable."]
    #[inline(always)]
    pub fn reg_sram_sub_ram_dvse(&mut self) -> RegSramSubRamDvseW<'_, SramParmSpec> {
        RegSramSubRamDvseW::new(self, 28)
    }
    #[doc = "Bit 29 - Subsystem RAM Nap mode enable."]
    #[inline(always)]
    pub fn reg_sram_sub_ram_nap(&mut self) -> RegSramSubRamNapW<'_, SramParmSpec> {
        RegSramSubRamNapW::new(self, 29)
    }
}
#[doc = "SRAM Power and DVS (Dynamic Voltage Scaling) Parameters\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_parm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_parm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramParmSpec;
impl crate::RegisterSpec for SramParmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_parm::R`](R) reader structure"]
impl crate::Readable for SramParmSpec {}
#[doc = "`write(|w| ..)` method takes [`sram_parm::W`](W) writer structure"]
impl crate::Writable for SramParmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAM_PARM to value 0x0c0c_0c0c"]
impl crate::Resettable for SramParmSpec {
    const RESET_VALUE: u32 = 0x0c0c_0c0c;
}
