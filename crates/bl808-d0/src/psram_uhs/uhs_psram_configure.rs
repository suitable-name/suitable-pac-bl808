#[doc = "Register `UHS_PSRAM_CONFIGURE` reader"]
pub type R = crate::R<UhsPsramConfigureSpec>;
#[doc = "Register `UHS_PSRAM_CONFIGURE` writer"]
pub type W = crate::W<UhsPsramConfigureSpec>;
#[doc = "Field `reg_uhs_latency` reader - UHS read/write latency"]
pub type RegUhsLatencyR = crate::FieldReader;
#[doc = "Field `reg_uhs_latency` writer - UHS read/write latency"]
pub type RegUhsLatencyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `reg_uhs_drive_st` reader - UHS output drive strength"]
pub type RegUhsDriveStR = crate::FieldReader;
#[doc = "Field `reg_uhs_drive_st` writer - UHS output drive strength"]
pub type RegUhsDriveStW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `reg_uhs_bl_16` reader - Burst length 16 selection"]
pub type RegUhsBl16R = crate::BitReader;
#[doc = "Field `reg_uhs_bl_16` writer - Burst length 16 selection"]
pub type RegUhsBl16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_uhs_bl_32` reader - Burst length 32 selection"]
pub type RegUhsBl32R = crate::BitReader;
#[doc = "Field `reg_uhs_bl_32` writer - Burst length 32 selection"]
pub type RegUhsBl32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `reg_uhs_bl_64` reader - Burst length 64 selection"]
pub type RegUhsBl64R = crate::BitReader;
#[doc = "Field `reg_uhs_bl_64` writer - Burst length 64 selection"]
pub type RegUhsBl64W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - UHS read/write latency"]
    #[inline(always)]
    pub fn reg_uhs_latency(&self) -> RegUhsLatencyR {
        RegUhsLatencyR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - UHS output drive strength"]
    #[inline(always)]
    pub fn reg_uhs_drive_st(&self) -> RegUhsDriveStR {
        RegUhsDriveStR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Burst length 16 selection"]
    #[inline(always)]
    pub fn reg_uhs_bl_16(&self) -> RegUhsBl16R {
        RegUhsBl16R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Burst length 32 selection"]
    #[inline(always)]
    pub fn reg_uhs_bl_32(&self) -> RegUhsBl32R {
        RegUhsBl32R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Burst length 64 selection"]
    #[inline(always)]
    pub fn reg_uhs_bl_64(&self) -> RegUhsBl64R {
        RegUhsBl64R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - UHS read/write latency"]
    #[inline(always)]
    pub fn reg_uhs_latency(&mut self) -> RegUhsLatencyW<'_, UhsPsramConfigureSpec> {
        RegUhsLatencyW::new(self, 0)
    }
    #[doc = "Bits 4:7 - UHS output drive strength"]
    #[inline(always)]
    pub fn reg_uhs_drive_st(&mut self) -> RegUhsDriveStW<'_, UhsPsramConfigureSpec> {
        RegUhsDriveStW::new(self, 4)
    }
    #[doc = "Bit 8 - Burst length 16 selection"]
    #[inline(always)]
    pub fn reg_uhs_bl_16(&mut self) -> RegUhsBl16W<'_, UhsPsramConfigureSpec> {
        RegUhsBl16W::new(self, 8)
    }
    #[doc = "Bit 9 - Burst length 32 selection"]
    #[inline(always)]
    pub fn reg_uhs_bl_32(&mut self) -> RegUhsBl32W<'_, UhsPsramConfigureSpec> {
        RegUhsBl32W::new(self, 9)
    }
    #[doc = "Bit 10 - Burst length 64 selection"]
    #[inline(always)]
    pub fn reg_uhs_bl_64(&mut self) -> RegUhsBl64W<'_, UhsPsramConfigureSpec> {
        RegUhsBl64W::new(self, 10)
    }
}
#[doc = "UHS PSRAM Device Parameters Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs_psram_configure::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs_psram_configure::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsPsramConfigureSpec;
impl crate::RegisterSpec for UhsPsramConfigureSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs_psram_configure::R`](R) reader structure"]
impl crate::Readable for UhsPsramConfigureSpec {}
#[doc = "`write(|w| ..)` method takes [`uhs_psram_configure::W`](W) writer structure"]
impl crate::Writable for UhsPsramConfigureSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHS_PSRAM_CONFIGURE to value 0xa5"]
impl crate::Resettable for UhsPsramConfigureSpec {
    const RESET_VALUE: u32 = 0xa5;
}
