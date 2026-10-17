#[doc = "Register `LZ4_CONFIG` reader"]
pub type R = crate::R<Lz4ConfigSpec>;
#[doc = "Register `LZ4_CONFIG` writer"]
pub type W = crate::W<Lz4ConfigSpec>;
#[doc = "Field `lz4_en` reader - Enable LZ4 decompression"]
pub type Lz4EnR = crate::BitReader;
#[doc = "Field `lz4_en` writer - Enable LZ4 decompression"]
pub type Lz4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lz4_suspend` reader - Suspend LZ4 decompression"]
pub type Lz4SuspendR = crate::BitReader;
#[doc = "Field `lz4_suspend` writer - Suspend LZ4 decompression"]
pub type Lz4SuspendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lz4_checksum` reader - Enable LZ4 checksum verification"]
pub type Lz4ChecksumR = crate::BitReader;
#[doc = "Field `lz4_checksum` writer - Enable LZ4 checksum verification"]
pub type Lz4ChecksumW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable LZ4 decompression"]
    #[inline(always)]
    pub fn lz4_en(&self) -> Lz4EnR {
        Lz4EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Suspend LZ4 decompression"]
    #[inline(always)]
    pub fn lz4_suspend(&self) -> Lz4SuspendR {
        Lz4SuspendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable LZ4 checksum verification"]
    #[inline(always)]
    pub fn lz4_checksum(&self) -> Lz4ChecksumR {
        Lz4ChecksumR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable LZ4 decompression"]
    #[inline(always)]
    pub fn lz4_en(&mut self) -> Lz4EnW<'_, Lz4ConfigSpec> {
        Lz4EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Suspend LZ4 decompression"]
    #[inline(always)]
    pub fn lz4_suspend(&mut self) -> Lz4SuspendW<'_, Lz4ConfigSpec> {
        Lz4SuspendW::new(self, 1)
    }
    #[doc = "Bit 4 - Enable LZ4 checksum verification"]
    #[inline(always)]
    pub fn lz4_checksum(&mut self) -> Lz4ChecksumW<'_, Lz4ConfigSpec> {
        Lz4ChecksumW::new(self, 4)
    }
}
#[doc = "LZ4 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lz4_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lz4_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lz4ConfigSpec;
impl crate::RegisterSpec for Lz4ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lz4_config::R`](R) reader structure"]
impl crate::Readable for Lz4ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`lz4_config::W`](W) writer structure"]
impl crate::Writable for Lz4ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LZ4_CONFIG to value 0"]
impl crate::Resettable for Lz4ConfigSpec {}
