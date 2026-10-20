// @generated
impl serde::Serialize for ClientStreamHeader {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.stream_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sown.protocol.v1.ClientStreamHeader", len)?;
        if self.stream_id != 0 {
            let v = StreamId::try_from(self.stream_id)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.stream_id)))?;
            struct_ser.serialize_field("streamId", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClientStreamHeader {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "stream_id",
            "streamId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StreamId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "streamId" | "stream_id" => Ok(GeneratedField::StreamId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClientStreamHeader;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sown.protocol.v1.ClientStreamHeader")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClientStreamHeader, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut stream_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StreamId => {
                            if stream_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("streamId"));
                            }
                            stream_id__ = Some(map_.next_value::<StreamId>()? as i32);
                        }
                    }
                }
                Ok(ClientStreamHeader {
                    stream_id: stream_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sown.protocol.v1.ClientStreamHeader", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ControlClientHello {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.protocol_version != 0 {
            len += 1;
        }
        if self.username.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sown.protocol.v1.ControlClientHello", len)?;
        if self.protocol_version != 0 {
            struct_ser.serialize_field("protocolVersion", &self.protocol_version)?;
        }
        if let Some(v) = self.username.as_ref() {
            struct_ser.serialize_field("username", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ControlClientHello {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "protocol_version",
            "protocolVersion",
            "username",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProtocolVersion,
            Username,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "protocolVersion" | "protocol_version" => Ok(GeneratedField::ProtocolVersion),
                            "username" => Ok(GeneratedField::Username),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ControlClientHello;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sown.protocol.v1.ControlClientHello")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ControlClientHello, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut protocol_version__ = None;
                let mut username__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProtocolVersion => {
                            if protocol_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocolVersion"));
                            }
                            protocol_version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ControlClientHello {
                    protocol_version: protocol_version__.unwrap_or_default(),
                    username: username__,
                })
            }
        }
        deserializer.deserialize_struct("sown.protocol.v1.ControlClientHello", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ControlClientJoin {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("sown.protocol.v1.ControlClientJoin", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ControlClientJoin {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ControlClientJoin;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sown.protocol.v1.ControlClientJoin")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ControlClientJoin, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(ControlClientJoin {
                })
            }
        }
        deserializer.deserialize_struct("sown.protocol.v1.ControlClientJoin", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ControlEnvelope {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.payload.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sown.protocol.v1.ControlEnvelope", len)?;
        if let Some(v) = self.payload.as_ref() {
            match v {
                control_envelope::Payload::ClientHello(v) => {
                    struct_ser.serialize_field("clientHello", v)?;
                }
                control_envelope::Payload::ServerWelcome(v) => {
                    struct_ser.serialize_field("serverWelcome", v)?;
                }
                control_envelope::Payload::ServerError(v) => {
                    struct_ser.serialize_field("serverError", v)?;
                }
                control_envelope::Payload::ClientJoin(v) => {
                    struct_ser.serialize_field("clientJoin", v)?;
                }
                control_envelope::Payload::ServerJoined(v) => {
                    struct_ser.serialize_field("serverJoined", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ControlEnvelope {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "client_hello",
            "clientHello",
            "server_welcome",
            "serverWelcome",
            "server_error",
            "serverError",
            "client_join",
            "clientJoin",
            "server_joined",
            "serverJoined",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClientHello,
            ServerWelcome,
            ServerError,
            ClientJoin,
            ServerJoined,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "clientHello" | "client_hello" => Ok(GeneratedField::ClientHello),
                            "serverWelcome" | "server_welcome" => Ok(GeneratedField::ServerWelcome),
                            "serverError" | "server_error" => Ok(GeneratedField::ServerError),
                            "clientJoin" | "client_join" => Ok(GeneratedField::ClientJoin),
                            "serverJoined" | "server_joined" => Ok(GeneratedField::ServerJoined),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ControlEnvelope;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sown.protocol.v1.ControlEnvelope")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ControlEnvelope, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut payload__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClientHello => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientHello"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(control_envelope::Payload::ClientHello)
;
                        }
                        GeneratedField::ServerWelcome => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverWelcome"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(control_envelope::Payload::ServerWelcome)
;
                        }
                        GeneratedField::ServerError => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverError"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(control_envelope::Payload::ServerError)
;
                        }
                        GeneratedField::ClientJoin => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clientJoin"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(control_envelope::Payload::ClientJoin)
;
                        }
                        GeneratedField::ServerJoined => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverJoined"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(control_envelope::Payload::ServerJoined)
;
                        }
                    }
                }
                Ok(ControlEnvelope {
                    payload: payload__,
                })
            }
        }
        deserializer.deserialize_struct("sown.protocol.v1.ControlEnvelope", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ControlServerError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sown.protocol.v1.ControlServerError", len)?;
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ControlServerError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Message,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ControlServerError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sown.protocol.v1.ControlServerError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ControlServerError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ControlServerError {
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sown.protocol.v1.ControlServerError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ControlServerJoined {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.player_entity_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sown.protocol.v1.ControlServerJoined", len)?;
        if self.player_entity_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("playerEntityId", ToString::to_string(&self.player_entity_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ControlServerJoined {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "player_entity_id",
            "playerEntityId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PlayerEntityId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "playerEntityId" | "player_entity_id" => Ok(GeneratedField::PlayerEntityId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ControlServerJoined;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sown.protocol.v1.ControlServerJoined")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ControlServerJoined, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut player_entity_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PlayerEntityId => {
                            if player_entity_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("playerEntityId"));
                            }
                            player_entity_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ControlServerJoined {
                    player_entity_id: player_entity_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sown.protocol.v1.ControlServerJoined", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ControlServerWelcome {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.protocol_version != 0 {
            len += 1;
        }
        if self.peer_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sown.protocol.v1.ControlServerWelcome", len)?;
        if self.protocol_version != 0 {
            struct_ser.serialize_field("protocolVersion", &self.protocol_version)?;
        }
        if self.peer_id != 0 {
            struct_ser.serialize_field("peerId", &self.peer_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ControlServerWelcome {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "protocol_version",
            "protocolVersion",
            "peer_id",
            "peerId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProtocolVersion,
            PeerId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "protocolVersion" | "protocol_version" => Ok(GeneratedField::ProtocolVersion),
                            "peerId" | "peer_id" => Ok(GeneratedField::PeerId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ControlServerWelcome;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sown.protocol.v1.ControlServerWelcome")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ControlServerWelcome, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut protocol_version__ = None;
                let mut peer_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ProtocolVersion => {
                            if protocol_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocolVersion"));
                            }
                            protocol_version__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PeerId => {
                            if peer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("peerId"));
                            }
                            peer_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ControlServerWelcome {
                    protocol_version: protocol_version__.unwrap_or_default(),
                    peer_id: peer_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sown.protocol.v1.ControlServerWelcome", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DatagramClientLocalPlayerMove {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.transform.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sown.protocol.v1.DatagramClientLocalPlayerMove", len)?;
        if let Some(v) = self.transform.as_ref() {
            struct_ser.serialize_field("transform", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DatagramClientLocalPlayerMove {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transform",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Transform,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "transform" => Ok(GeneratedField::Transform),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DatagramClientLocalPlayerMove;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sown.protocol.v1.DatagramClientLocalPlayerMove")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DatagramClientLocalPlayerMove, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transform__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Transform => {
                            if transform__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transform"));
                            }
                            transform__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DatagramClientLocalPlayerMove {
                    transform: transform__,
                })
            }
        }
        deserializer.deserialize_struct("sown.protocol.v1.DatagramClientLocalPlayerMove", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DatagramEnvelope {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.payload.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sown.protocol.v1.DatagramEnvelope", len)?;
        if let Some(v) = self.payload.as_ref() {
            match v {
                datagram_envelope::Payload::LocalPlayerMove(v) => {
                    struct_ser.serialize_field("localPlayerMove", v)?;
                }
                datagram_envelope::Payload::PeersSnapshot(v) => {
                    struct_ser.serialize_field("peersSnapshot", v)?;
                }
                datagram_envelope::Payload::EntitiesSnapshot(v) => {
                    struct_ser.serialize_field("entitiesSnapshot", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DatagramEnvelope {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "local_player_move",
            "localPlayerMove",
            "peers_snapshot",
            "peersSnapshot",
            "entities_snapshot",
            "entitiesSnapshot",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LocalPlayerMove,
            PeersSnapshot,
            EntitiesSnapshot,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "localPlayerMove" | "local_player_move" => Ok(GeneratedField::LocalPlayerMove),
                            "peersSnapshot" | "peers_snapshot" => Ok(GeneratedField::PeersSnapshot),
                            "entitiesSnapshot" | "entities_snapshot" => Ok(GeneratedField::EntitiesSnapshot),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DatagramEnvelope;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sown.protocol.v1.DatagramEnvelope")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DatagramEnvelope, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut payload__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::LocalPlayerMove => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("localPlayerMove"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(datagram_envelope::Payload::LocalPlayerMove)
;
                        }
                        GeneratedField::PeersSnapshot => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("peersSnapshot"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(datagram_envelope::Payload::PeersSnapshot)
;
                        }
                        GeneratedField::EntitiesSnapshot => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entitiesSnapshot"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(datagram_envelope::Payload::EntitiesSnapshot)
;
                        }
                    }
                }
                Ok(DatagramEnvelope {
                    payload: payload__,
                })
            }
        }
        deserializer.deserialize_struct("sown.protocol.v1.DatagramEnvelope", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EntitySnapshot {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.entity_id != 0 {
            len += 1;
        }
        if self.owner_peer_id != 0 {
            len += 1;
        }
        if self.transform.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sown.protocol.v1.EntitySnapshot", len)?;
        if self.entity_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("entityId", ToString::to_string(&self.entity_id).as_str())?;
        }
        if self.owner_peer_id != 0 {
            struct_ser.serialize_field("ownerPeerId", &self.owner_peer_id)?;
        }
        if let Some(v) = self.transform.as_ref() {
            struct_ser.serialize_field("transform", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EntitySnapshot {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity_id",
            "entityId",
            "owner_peer_id",
            "ownerPeerId",
            "transform",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntityId,
            OwnerPeerId,
            Transform,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "entityId" | "entity_id" => Ok(GeneratedField::EntityId),
                            "ownerPeerId" | "owner_peer_id" => Ok(GeneratedField::OwnerPeerId),
                            "transform" => Ok(GeneratedField::Transform),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EntitySnapshot;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sown.protocol.v1.EntitySnapshot")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EntitySnapshot, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity_id__ = None;
                let mut owner_peer_id__ = None;
                let mut transform__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntityId => {
                            if entity_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityId"));
                            }
                            entity_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::OwnerPeerId => {
                            if owner_peer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ownerPeerId"));
                            }
                            owner_peer_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Transform => {
                            if transform__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transform"));
                            }
                            transform__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EntitySnapshot {
                    entity_id: entity_id__.unwrap_or_default(),
                    owner_peer_id: owner_peer_id__.unwrap_or_default(),
                    transform: transform__,
                })
            }
        }
        deserializer.deserialize_struct("sown.protocol.v1.EntitySnapshot", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventEnvelope {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.payload.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sown.protocol.v1.EventEnvelope", len)?;
        if let Some(v) = self.payload.as_ref() {
            match v {
                event_envelope::Payload::PeerAdmitted(v) => {
                    struct_ser.serialize_field("peerAdmitted", v)?;
                }
                event_envelope::Payload::PeerRemoved(v) => {
                    struct_ser.serialize_field("peerRemoved", v)?;
                }
                event_envelope::Payload::EntityCreated(v) => {
                    struct_ser.serialize_field("entityCreated", v)?;
                }
                event_envelope::Payload::EntityRemoved(v) => {
                    struct_ser.serialize_field("entityRemoved", v)?;
                }
                event_envelope::Payload::InitialPeersSnapshot(v) => {
                    struct_ser.serialize_field("initialPeersSnapshot", v)?;
                }
                event_envelope::Payload::InitialEntitiesSnapshot(v) => {
                    struct_ser.serialize_field("initialEntitiesSnapshot", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventEnvelope {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "peer_admitted",
            "peerAdmitted",
            "peer_removed",
            "peerRemoved",
            "entity_created",
            "entityCreated",
            "entity_removed",
            "entityRemoved",
            "initial_peers_snapshot",
            "initialPeersSnapshot",
            "initial_entities_snapshot",
            "initialEntitiesSnapshot",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PeerAdmitted,
            PeerRemoved,
            EntityCreated,
            EntityRemoved,
            InitialPeersSnapshot,
            InitialEntitiesSnapshot,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "peerAdmitted" | "peer_admitted" => Ok(GeneratedField::PeerAdmitted),
                            "peerRemoved" | "peer_removed" => Ok(GeneratedField::PeerRemoved),
                            "entityCreated" | "entity_created" => Ok(GeneratedField::EntityCreated),
                            "entityRemoved" | "entity_removed" => Ok(GeneratedField::EntityRemoved),
                            "initialPeersSnapshot" | "initial_peers_snapshot" => Ok(GeneratedField::InitialPeersSnapshot),
                            "initialEntitiesSnapshot" | "initial_entities_snapshot" => Ok(GeneratedField::InitialEntitiesSnapshot),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventEnvelope;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sown.protocol.v1.EventEnvelope")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventEnvelope, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut payload__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PeerAdmitted => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("peerAdmitted"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(event_envelope::Payload::PeerAdmitted)
;
                        }
                        GeneratedField::PeerRemoved => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("peerRemoved"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(event_envelope::Payload::PeerRemoved)
;
                        }
                        GeneratedField::EntityCreated => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityCreated"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(event_envelope::Payload::EntityCreated)
;
                        }
                        GeneratedField::EntityRemoved => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityRemoved"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(event_envelope::Payload::EntityRemoved)
;
                        }
                        GeneratedField::InitialPeersSnapshot => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialPeersSnapshot"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(event_envelope::Payload::InitialPeersSnapshot)
;
                        }
                        GeneratedField::InitialEntitiesSnapshot => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("initialEntitiesSnapshot"));
                            }
                            payload__ = map_.next_value::<::std::option::Option<_>>()?.map(event_envelope::Payload::InitialEntitiesSnapshot)
;
                        }
                    }
                }
                Ok(EventEnvelope {
                    payload: payload__,
                })
            }
        }
        deserializer.deserialize_struct("sown.protocol.v1.EventEnvelope", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventServerEntityCreated {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.snapshot.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sown.protocol.v1.EventServerEntityCreated", len)?;
        if let Some(v) = self.snapshot.as_ref() {
            struct_ser.serialize_field("snapshot", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventServerEntityCreated {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "snapshot",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Snapshot,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "snapshot" => Ok(GeneratedField::Snapshot),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventServerEntityCreated;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sown.protocol.v1.EventServerEntityCreated")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventServerEntityCreated, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut snapshot__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Snapshot => {
                            if snapshot__.is_some() {
                                return Err(serde::de::Error::duplicate_field("snapshot"));
                            }
                            snapshot__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EventServerEntityCreated {
                    snapshot: snapshot__,
                })
            }
        }
        deserializer.deserialize_struct("sown.protocol.v1.EventServerEntityCreated", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventServerEntityRemoved {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.entity_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sown.protocol.v1.EventServerEntityRemoved", len)?;
        if self.entity_id != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("entityId", ToString::to_string(&self.entity_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventServerEntityRemoved {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entity_id",
            "entityId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntityId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "entityId" | "entity_id" => Ok(GeneratedField::EntityId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventServerEntityRemoved;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sown.protocol.v1.EventServerEntityRemoved")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventServerEntityRemoved, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entity_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntityId => {
                            if entity_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityId"));
                            }
                            entity_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(EventServerEntityRemoved {
                    entity_id: entity_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sown.protocol.v1.EventServerEntityRemoved", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventServerPeerAdmitted {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.snapshot.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sown.protocol.v1.EventServerPeerAdmitted", len)?;
        if let Some(v) = self.snapshot.as_ref() {
            struct_ser.serialize_field("snapshot", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventServerPeerAdmitted {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "snapshot",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Snapshot,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "snapshot" => Ok(GeneratedField::Snapshot),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventServerPeerAdmitted;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sown.protocol.v1.EventServerPeerAdmitted")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventServerPeerAdmitted, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut snapshot__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Snapshot => {
                            if snapshot__.is_some() {
                                return Err(serde::de::Error::duplicate_field("snapshot"));
                            }
                            snapshot__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EventServerPeerAdmitted {
                    snapshot: snapshot__,
                })
            }
        }
        deserializer.deserialize_struct("sown.protocol.v1.EventServerPeerAdmitted", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventServerPeerRemoved {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.peer_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sown.protocol.v1.EventServerPeerRemoved", len)?;
        if self.peer_id != 0 {
            struct_ser.serialize_field("peerId", &self.peer_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventServerPeerRemoved {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "peer_id",
            "peerId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PeerId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "peerId" | "peer_id" => Ok(GeneratedField::PeerId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventServerPeerRemoved;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sown.protocol.v1.EventServerPeerRemoved")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventServerPeerRemoved, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut peer_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PeerId => {
                            if peer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("peerId"));
                            }
                            peer_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(EventServerPeerRemoved {
                    peer_id: peer_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sown.protocol.v1.EventServerPeerRemoved", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PeerSnapshot {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.peer_id != 0 {
            len += 1;
        }
        if self.username.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sown.protocol.v1.PeerSnapshot", len)?;
        if self.peer_id != 0 {
            struct_ser.serialize_field("peerId", &self.peer_id)?;
        }
        if let Some(v) = self.username.as_ref() {
            struct_ser.serialize_field("username", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PeerSnapshot {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "peer_id",
            "peerId",
            "username",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PeerId,
            Username,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "peerId" | "peer_id" => Ok(GeneratedField::PeerId),
                            "username" => Ok(GeneratedField::Username),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PeerSnapshot;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sown.protocol.v1.PeerSnapshot")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PeerSnapshot, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut peer_id__ = None;
                let mut username__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PeerId => {
                            if peer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("peerId"));
                            }
                            peer_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("username"));
                            }
                            username__ = map_.next_value()?;
                        }
                    }
                }
                Ok(PeerSnapshot {
                    peer_id: peer_id__.unwrap_or_default(),
                    username: username__,
                })
            }
        }
        deserializer.deserialize_struct("sown.protocol.v1.PeerSnapshot", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Quaternion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.x != 0. {
            len += 1;
        }
        if self.y != 0. {
            len += 1;
        }
        if self.z != 0. {
            len += 1;
        }
        if self.w != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sown.protocol.v1.Quaternion", len)?;
        if self.x != 0. {
            struct_ser.serialize_field("x", &self.x)?;
        }
        if self.y != 0. {
            struct_ser.serialize_field("y", &self.y)?;
        }
        if self.z != 0. {
            struct_ser.serialize_field("z", &self.z)?;
        }
        if self.w != 0. {
            struct_ser.serialize_field("w", &self.w)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Quaternion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "x",
            "y",
            "z",
            "w",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            X,
            Y,
            Z,
            W,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "x" => Ok(GeneratedField::X),
                            "y" => Ok(GeneratedField::Y),
                            "z" => Ok(GeneratedField::Z),
                            "w" => Ok(GeneratedField::W),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Quaternion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sown.protocol.v1.Quaternion")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Quaternion, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut x__ = None;
                let mut y__ = None;
                let mut z__ = None;
                let mut w__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::X => {
                            if x__.is_some() {
                                return Err(serde::de::Error::duplicate_field("x"));
                            }
                            x__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Y => {
                            if y__.is_some() {
                                return Err(serde::de::Error::duplicate_field("y"));
                            }
                            y__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Z => {
                            if z__.is_some() {
                                return Err(serde::de::Error::duplicate_field("z"));
                            }
                            z__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::W => {
                            if w__.is_some() {
                                return Err(serde::de::Error::duplicate_field("w"));
                            }
                            w__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Quaternion {
                    x: x__.unwrap_or_default(),
                    y: y__.unwrap_or_default(),
                    z: z__.unwrap_or_default(),
                    w: w__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sown.protocol.v1.Quaternion", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServerEntitiesSnapshot {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.server_tick != 0 {
            len += 1;
        }
        if !self.snapshot.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sown.protocol.v1.ServerEntitiesSnapshot", len)?;
        if self.server_tick != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("serverTick", ToString::to_string(&self.server_tick).as_str())?;
        }
        if !self.snapshot.is_empty() {
            struct_ser.serialize_field("snapshot", &self.snapshot)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServerEntitiesSnapshot {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "server_tick",
            "serverTick",
            "snapshot",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ServerTick,
            Snapshot,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "serverTick" | "server_tick" => Ok(GeneratedField::ServerTick),
                            "snapshot" => Ok(GeneratedField::Snapshot),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServerEntitiesSnapshot;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sown.protocol.v1.ServerEntitiesSnapshot")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ServerEntitiesSnapshot, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut server_tick__ = None;
                let mut snapshot__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ServerTick => {
                            if server_tick__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverTick"));
                            }
                            server_tick__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Snapshot => {
                            if snapshot__.is_some() {
                                return Err(serde::de::Error::duplicate_field("snapshot"));
                            }
                            snapshot__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ServerEntitiesSnapshot {
                    server_tick: server_tick__.unwrap_or_default(),
                    snapshot: snapshot__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sown.protocol.v1.ServerEntitiesSnapshot", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServerPeersSnapshot {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.server_tick != 0 {
            len += 1;
        }
        if !self.snapshot.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sown.protocol.v1.ServerPeersSnapshot", len)?;
        if self.server_tick != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("serverTick", ToString::to_string(&self.server_tick).as_str())?;
        }
        if !self.snapshot.is_empty() {
            struct_ser.serialize_field("snapshot", &self.snapshot)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServerPeersSnapshot {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "server_tick",
            "serverTick",
            "snapshot",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ServerTick,
            Snapshot,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "serverTick" | "server_tick" => Ok(GeneratedField::ServerTick),
                            "snapshot" => Ok(GeneratedField::Snapshot),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServerPeersSnapshot;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sown.protocol.v1.ServerPeersSnapshot")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ServerPeersSnapshot, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut server_tick__ = None;
                let mut snapshot__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ServerTick => {
                            if server_tick__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serverTick"));
                            }
                            server_tick__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Snapshot => {
                            if snapshot__.is_some() {
                                return Err(serde::de::Error::duplicate_field("snapshot"));
                            }
                            snapshot__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ServerPeersSnapshot {
                    server_tick: server_tick__.unwrap_or_default(),
                    snapshot: snapshot__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sown.protocol.v1.ServerPeersSnapshot", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StreamId {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Control => "CONTROL",
            Self::Events => "EVENTS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for StreamId {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CONTROL",
            "EVENTS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StreamId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "CONTROL" => Ok(StreamId::Control),
                    "EVENTS" => Ok(StreamId::Events),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Transform {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.translation.is_some() {
            len += 1;
        }
        if self.rotation.is_some() {
            len += 1;
        }
        if self.scale.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sown.protocol.v1.Transform", len)?;
        if let Some(v) = self.translation.as_ref() {
            struct_ser.serialize_field("translation", v)?;
        }
        if let Some(v) = self.rotation.as_ref() {
            struct_ser.serialize_field("rotation", v)?;
        }
        if let Some(v) = self.scale.as_ref() {
            struct_ser.serialize_field("scale", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Transform {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "translation",
            "rotation",
            "scale",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Translation,
            Rotation,
            Scale,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "translation" => Ok(GeneratedField::Translation),
                            "rotation" => Ok(GeneratedField::Rotation),
                            "scale" => Ok(GeneratedField::Scale),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Transform;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sown.protocol.v1.Transform")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Transform, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut translation__ = None;
                let mut rotation__ = None;
                let mut scale__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Translation => {
                            if translation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("translation"));
                            }
                            translation__ = map_.next_value()?;
                        }
                        GeneratedField::Rotation => {
                            if rotation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rotation"));
                            }
                            rotation__ = map_.next_value()?;
                        }
                        GeneratedField::Scale => {
                            if scale__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scale"));
                            }
                            scale__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Transform {
                    translation: translation__,
                    rotation: rotation__,
                    scale: scale__,
                })
            }
        }
        deserializer.deserialize_struct("sown.protocol.v1.Transform", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Vector3 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.x != 0. {
            len += 1;
        }
        if self.y != 0. {
            len += 1;
        }
        if self.z != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("sown.protocol.v1.Vector3", len)?;
        if self.x != 0. {
            struct_ser.serialize_field("x", &self.x)?;
        }
        if self.y != 0. {
            struct_ser.serialize_field("y", &self.y)?;
        }
        if self.z != 0. {
            struct_ser.serialize_field("z", &self.z)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Vector3 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "x",
            "y",
            "z",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            X,
            Y,
            Z,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "x" => Ok(GeneratedField::X),
                            "y" => Ok(GeneratedField::Y),
                            "z" => Ok(GeneratedField::Z),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Vector3;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct sown.protocol.v1.Vector3")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Vector3, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut x__ = None;
                let mut y__ = None;
                let mut z__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::X => {
                            if x__.is_some() {
                                return Err(serde::de::Error::duplicate_field("x"));
                            }
                            x__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Y => {
                            if y__.is_some() {
                                return Err(serde::de::Error::duplicate_field("y"));
                            }
                            y__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Z => {
                            if z__.is_some() {
                                return Err(serde::de::Error::duplicate_field("z"));
                            }
                            z__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Vector3 {
                    x: x__.unwrap_or_default(),
                    y: y__.unwrap_or_default(),
                    z: z__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("sown.protocol.v1.Vector3", FIELDS, GeneratedVisitor)
    }
}
