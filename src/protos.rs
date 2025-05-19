pub mod animations_ {
    pub mod SetAnimation_ {
        #[derive(Debug, PartialEq, Clone)]
        pub enum Animation {
            RainbowAnimation(super::list_::rainbow_::RainbowAnimation),
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct SetAnimation {
        pub r#animation: ::core::option::Option<SetAnimation_::Animation>,
    }
    impl ::core::default::Default for SetAnimation {
        fn default() -> Self {
            Self {
                r#animation: ::core::default::Default::default(),
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
                        let mut_ref = loop {
                            if let ::core::option::Option::Some(variant) = &mut self
                                .r#animation
                            {
                                if let SetAnimation_::Animation::RainbowAnimation(
                                    variant,
                                ) = &mut *variant {
                                    break &mut *variant;
                                }
                            }
                            self.r#animation = ::core::option::Option::Some(
                                SetAnimation_::Animation::RainbowAnimation(
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
            if let Some(oneof) = &self.r#animation {
                match &*oneof {
                    SetAnimation_::Animation::RainbowAnimation(val_ref) => {
                        let val_ref = &*val_ref;
                        encoder.encode_varint32(10u32)?;
                        val_ref.encode_len_delimited(encoder)?;
                    }
                }
            }
            Ok(())
        }
        fn compute_size(&self) -> usize {
            use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
            let mut size = 0;
            if let Some(oneof) = &self.r#animation {
                match &*oneof {
                    SetAnimation_::Animation::RainbowAnimation(val_ref) => {
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
    pub mod list_ {
        pub mod rainbow_ {
            #[derive(Debug, PartialEq, Clone)]
            pub struct RainbowAnimation {
                pub r#speed: f32,
                pub r#progressive: bool,
            }
            impl ::core::default::Default for RainbowAnimation {
                fn default() -> Self {
                    Self {
                        r#speed: ::core::default::Default::default(),
                        r#progressive: ::core::default::Default::default(),
                    }
                }
            }
            impl RainbowAnimation {}
            impl ::micropb::MessageDecode for RainbowAnimation {
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
            impl ::micropb::MessageEncode for RainbowAnimation {
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
}
pub mod sync_ {
    pub mod Packet_ {
        #[derive(Debug, PartialEq, Clone)]
        pub enum Packet {
            Sync(super::sync_::Sync),
            PingPong(super::ping_::PingPong),
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct Packet {
        pub r#packet: ::core::option::Option<Packet_::Packet>,
    }
    impl ::core::default::Default for Packet {
        fn default() -> Self {
            Self {
                r#packet: ::core::default::Default::default(),
            }
        }
    }
    impl Packet {}
    impl ::micropb::MessageDecode for Packet {
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
                        let mut_ref = loop {
                            if let ::core::option::Option::Some(variant) = &mut self
                                .r#packet
                            {
                                if let Packet_::Packet::Sync(variant) = &mut *variant {
                                    break &mut *variant;
                                }
                            }
                            self.r#packet = ::core::option::Option::Some(
                                Packet_::Packet::Sync(::core::default::Default::default()),
                            );
                        };
                        mut_ref.decode_len_delimited(decoder)?;
                    }
                    2u32 => {
                        let mut_ref = loop {
                            if let ::core::option::Option::Some(variant) = &mut self
                                .r#packet
                            {
                                if let Packet_::Packet::PingPong(variant) = &mut *variant {
                                    break &mut *variant;
                                }
                            }
                            self.r#packet = ::core::option::Option::Some(
                                Packet_::Packet::PingPong(
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
    impl ::micropb::MessageEncode for Packet {
        fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
            &self,
            encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
        ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
            use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
            if let Some(oneof) = &self.r#packet {
                match &*oneof {
                    Packet_::Packet::Sync(val_ref) => {
                        let val_ref = &*val_ref;
                        encoder.encode_varint32(10u32)?;
                        val_ref.encode_len_delimited(encoder)?;
                    }
                    Packet_::Packet::PingPong(val_ref) => {
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
            if let Some(oneof) = &self.r#packet {
                match &*oneof {
                    Packet_::Packet::Sync(val_ref) => {
                        let val_ref = &*val_ref;
                        size
                            += 1usize
                                + ::micropb::size::sizeof_len_record(
                                    val_ref.compute_size(),
                                );
                    }
                    Packet_::Packet::PingPong(val_ref) => {
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
    pub mod sync_ {
        pub mod Sync_ {
            #[derive(Debug, Default, PartialEq, Clone)]
            pub struct _Hazzer([u8; 1]);
            impl _Hazzer {
                ///Query presence of `set_animation`
                #[inline]
                pub fn r#set_animation(&self) -> bool {
                    (self.0[0] & 1) != 0
                }
                ///Set presence of `set_animation`
                #[inline]
                pub fn set_set_animation(&mut self) {
                    let elem = &mut self.0[0];
                    *elem |= 1;
                }
                ///Clear presence of `set_animation`
                #[inline]
                pub fn clear_set_animation(&mut self) {
                    let elem = &mut self.0[0];
                    *elem &= !1;
                }
                ///Builder method that sets the presence of `set_animation`. Useful for initializing the Hazzer.
                #[inline]
                pub fn init_set_animation(mut self) -> Self {
                    self.set_set_animation();
                    self
                }
            }
        }
        #[derive(Debug, PartialEq, Clone)]
        pub struct Sync {
            pub r#set_animation: super::super::animations_::SetAnimation,
            pub _has: Sync_::_Hazzer,
        }
        impl ::core::default::Default for Sync {
            fn default() -> Self {
                Self {
                    r#set_animation: ::core::default::Default::default(),
                    _has: ::core::default::Default::default(),
                }
            }
        }
        impl Sync {
            ///Return a reference to `set_animation` as an `Option`
            #[inline]
            pub fn r#set_animation(
                &self,
            ) -> ::core::option::Option<&super::super::animations_::SetAnimation> {
                self._has.r#set_animation().then_some(&self.r#set_animation)
            }
            ///Return a mutable reference to `set_animation` as an `Option`
            #[inline]
            pub fn mut_set_animation(
                &mut self,
            ) -> ::core::option::Option<&mut super::super::animations_::SetAnimation> {
                self._has.r#set_animation().then_some(&mut self.r#set_animation)
            }
            ///Set the value and presence of `set_animation`
            #[inline]
            pub fn set_set_animation(
                &mut self,
                value: super::super::animations_::SetAnimation,
            ) {
                self._has.set_set_animation();
                self.r#set_animation = value.into();
            }
            ///Clear the presence of `set_animation`
            #[inline]
            pub fn clear_set_animation(&mut self) {
                self._has.clear_set_animation();
            }
        }
        impl ::micropb::MessageDecode for Sync {
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
                            let mut_ref = &mut self.r#set_animation;
                            {
                                mut_ref.decode_len_delimited(decoder)?;
                            };
                            self._has.set_set_animation();
                        }
                        _ => {
                            decoder.skip_wire_value(tag.wire_type())?;
                        }
                    }
                }
                Ok(())
            }
        }
        impl ::micropb::MessageEncode for Sync {
            fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                &self,
                encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
            ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                {
                    if let Some(val_ref) = self.r#set_animation() {
                        encoder.encode_varint32(10u32)?;
                        val_ref.encode_len_delimited(encoder)?;
                    }
                }
                Ok(())
            }
            fn compute_size(&self) -> usize {
                use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                let mut size = 0;
                {
                    if let Some(val_ref) = self.r#set_animation() {
                        size
                            += 1usize
                                + ::micropb::size::sizeof_len_record(
                                    val_ref.compute_size(),
                                );
                    }
                }
                size
            }
        }
    }
    pub mod ping_ {
        #[derive(Debug, PartialEq, Clone)]
        pub struct PingPong {
            pub r#slave_device_id: u32,
            pub r#slave_timestamp: u64,
            pub r#master_timestamp: u64,
        }
        impl ::core::default::Default for PingPong {
            fn default() -> Self {
                Self {
                    r#slave_device_id: ::core::default::Default::default(),
                    r#slave_timestamp: ::core::default::Default::default(),
                    r#master_timestamp: ::core::default::Default::default(),
                }
            }
        }
        impl PingPong {}
        impl ::micropb::MessageDecode for PingPong {
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
                            let mut_ref = &mut self.r#slave_device_id;
                            {
                                let val = decoder.decode_varint32()?;
                                let val_ref = &val;
                                if *val_ref != 0 {
                                    *mut_ref = val as _;
                                }
                            };
                        }
                        2u32 => {
                            let mut_ref = &mut self.r#slave_timestamp;
                            {
                                let val = decoder.decode_varint64()?;
                                let val_ref = &val;
                                if *val_ref != 0 {
                                    *mut_ref = val as _;
                                }
                            };
                        }
                        3u32 => {
                            let mut_ref = &mut self.r#master_timestamp;
                            {
                                let val = decoder.decode_varint64()?;
                                let val_ref = &val;
                                if *val_ref != 0 {
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
        impl ::micropb::MessageEncode for PingPong {
            fn encode<IMPL_MICROPB_WRITE: ::micropb::PbWrite>(
                &self,
                encoder: &mut ::micropb::PbEncoder<IMPL_MICROPB_WRITE>,
            ) -> Result<(), IMPL_MICROPB_WRITE::Error> {
                use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                {
                    let val_ref = &self.r#slave_device_id;
                    if *val_ref != 0 {
                        encoder.encode_varint32(8u32)?;
                        encoder.encode_varint32(*val_ref as _)?;
                    }
                }
                {
                    let val_ref = &self.r#slave_timestamp;
                    if *val_ref != 0 {
                        encoder.encode_varint32(16u32)?;
                        encoder.encode_varint64(*val_ref as _)?;
                    }
                }
                {
                    let val_ref = &self.r#master_timestamp;
                    if *val_ref != 0 {
                        encoder.encode_varint32(24u32)?;
                        encoder.encode_varint64(*val_ref as _)?;
                    }
                }
                Ok(())
            }
            fn compute_size(&self) -> usize {
                use ::micropb::{PbVec, PbMap, PbString, FieldEncode};
                let mut size = 0;
                {
                    let val_ref = &self.r#slave_device_id;
                    if *val_ref != 0 {
                        size += 1usize + ::micropb::size::sizeof_varint32(*val_ref as _);
                    }
                }
                {
                    let val_ref = &self.r#slave_timestamp;
                    if *val_ref != 0 {
                        size += 1usize + ::micropb::size::sizeof_varint64(*val_ref as _);
                    }
                }
                {
                    let val_ref = &self.r#master_timestamp;
                    if *val_ref != 0 {
                        size += 1usize + ::micropb::size::sizeof_varint64(*val_ref as _);
                    }
                }
                size
            }
        }
    }
}
