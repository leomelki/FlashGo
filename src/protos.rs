pub mod animations_ {
    pub mod SetAnimation_ {
        #[derive(Debug, PartialEq, Clone)]
        pub enum Config {
            RainbowConfig(super::configs_::RainbowAnimationConfig),
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct SetAnimation {
        pub r#animation_type: AnimationType,
        pub r#config: ::core::option::Option<SetAnimation_::Config>,
    }
    impl ::core::default::Default for SetAnimation {
        fn default() -> Self {
            Self {
                r#animation_type: ::core::default::Default::default(),
                r#config: ::core::default::Default::default(),
            }
        }
    }
    impl SetAnimation {}
    impl ::micropb::MessageDecode for SetAnimation {
        fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
            &mut self,
            decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
            len: usize,
        ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
            use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
            let before = decoder.bytes_read();
            while decoder.bytes_read() - before < len {
                let tag = decoder.decode_tag()?;
                match tag.field_num() {
                    0 => return Err(::micropb::DecodeError::ZeroField),
                    1u32 => {
                        let mut_ref = &mut self.r#animation_type;
                        {
                            let val = decoder
                                .decode_int32()
                                .map(|n| AnimationType(n as _))?;
                            let val_ref = &val;
                            if val_ref.0 != 0 {
                                *mut_ref = val as _;
                            }
                        };
                    }
                    2u32 => {
                        let mut_ref = loop {
                            if let ::core::option::Option::Some(variant) = &mut self
                                .r#config
                            {
                                if let SetAnimation_::Config::RainbowConfig(variant) = &mut *variant {
                                    break &mut *variant;
                                }
                            }
                            self.r#config = ::core::option::Option::Some(
                                SetAnimation_::Config::RainbowConfig(
                                    ::core::default::Default::default(),
                                ),
                            );
                        };
                        mut_ref.decode_len_delimited(decoder)?;
                    }
                    _ => {
                        decoder.skip_wire_value(tag.wire_type())?;
                    }
                }
            }
            Ok(())
        }
    }
    impl ::micropb::MessageEncode for SetAnimation {
        fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
            &self,
            encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
        ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
            use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
            {
                let val_ref = &self.r#animation_type;
                if val_ref.0 != 0 {
                    encoder.encode_varint32(8u32)?;
                    encoder.encode_int32(val_ref.0 as _)?;
                }
            }
            if let Some(oneof) = &self.r#config {
                match &*oneof {
                    SetAnimation_::Config::RainbowConfig(val_ref) => {
                        let val_ref = &*val_ref;
                        encoder.encode_varint32(18u32)?;
                        val_ref.encode_len_delimited(encoder)?;
                    }
                }
            }
            Ok(())
        }
        fn compute_size(&self) -> usize {
            use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
            let mut size = 0;
            {
                let val_ref = &self.r#animation_type;
                if val_ref.0 != 0 {
                    size += 1usize + ::micropb::size::sizeof_int32(val_ref.0 as _);
                }
            }
            if let Some(oneof) = &self.r#config {
                match &*oneof {
                    SetAnimation_::Config::RainbowConfig(val_ref) => {
                        let val_ref = &*val_ref;
                        size
                            += 1usize
                                + ::micropb::size::sizeof_len_record(
                                    val_ref.compute_size(),
                                );
                    }
                }
            }
            size
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct AnimationType(pub i32);
    impl AnimationType {
        pub const Rainbow: Self = Self(0);
    }
    impl core::default::Default for AnimationType {
        fn default() -> Self {
            Self(0)
        }
    }
    impl core::convert::From<i32> for AnimationType {
        fn from(val: i32) -> Self {
            Self(val)
        }
    }
    pub mod configs_ {
        #[derive(Debug, PartialEq, Clone)]
        pub struct RainbowAnimationConfig {
            pub r#speed: f32,
            pub r#progressive: bool,
        }
        impl ::core::default::Default for RainbowAnimationConfig {
            fn default() -> Self {
                Self {
                    r#speed: ::core::default::Default::default(),
                    r#progressive: ::core::default::Default::default(),
                }
            }
        }
        impl RainbowAnimationConfig {}
        impl ::micropb::MessageDecode for RainbowAnimationConfig {
            fn decode<IMPL_MICROPB_READ: ::micropb::PbRead>(
                &mut self,
                decoder: &mut ::micropb::PbDecoder<IMPL_MICROPB_READ>,
                len: usize,
            ) -> Result<(), ::micropb::DecodeError<IMPL_MICROPB_READ::Error>> {
                use ::micropb::{PbVec, PbMap, PbString, FieldDecode};
                let before = decoder.bytes_read();
                while decoder.bytes_read() - before < len {
                    let tag = decoder.decode_tag()?;
                    match tag.field_num() {
                        0 => return Err(::micropb::DecodeError::ZeroField),
                        1u32 => {
                            let mut_ref = &mut self.r#speed;
                            {
                                let val = decoder.decode_float()?;
                                let val_ref = &val;
                                if *val_ref != 0.0 {
                                    *mut_ref = val as _;
                                }
                            };
                        }
                        2u32 => {
                            let mut_ref = &mut self.r#progressive;
                            {
                                let val = decoder.decode_bool()?;
                                let val_ref = &val;
                                if *val_ref {
                                    *mut_ref = val as _;
                                }
                            };
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        impl ::micropb::MessageEncode for RainbowAnimationConfig {
            fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                &self,
                encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
            ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                {
                    let val_ref = &self.r#speed;
                    if *val_ref != 0.0 {
                        encoder.encode_varint32(13u32)?;
                        encoder.encode_float(*val_ref)?;
                    }
                }
                {
                    let val_ref = &self.r#progressive;
                    if *val_ref {
                        encoder.encode_varint32(16u32)?;
                        encoder.encode_bool(*val_ref)?;
                    }
                }
                Ok(())
            }
            fn compute_size(&self) -> usize {
                use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                let mut size = 0;
                {
                    let val_ref = &self.r#speed;
                    if *val_ref != 0.0 {
                        size += 1usize + 4;
                    }
                }
                {
                    let val_ref = &self.r#progressive;
                    if *val_ref {
                        size += 1usize + 1;
                    }
                }
                size
            }
        }
    }
}
