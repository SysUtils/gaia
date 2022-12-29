macro_rules! notification {
    ($name: ident $idname: ident { $($selector: literal = $variant: ident($data: ty)),* }) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $name {
            $( $variant($data), )*
        }

        #[derive(Debug, Clone, Copy, TryFromPrimitive)]
        #[repr(u8)]
        pub enum $idname {
            $( $variant = $selector, )*
        }

        impl $name {
            pub fn read(command: u8, data: impl std::io::Read) -> std::io::Result<Self> {
                let cmd = command.try_into().map_err(|e| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Unknown NotificationType received: {e}"),
                    )
                })?;

                Ok(match cmd {
                    $($idname::$variant => Self::$variant(crate::traits::Payload::read(data)?),)*
                })
            }
        }
    };

    ($name: ident $idname: ident { $($selector: literal = $variant: ident($data: ty)),* } $unknown: ident($unknowndata: ty)) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $name {
            $( $variant($data), )*
            $unknown($unknowndata)
        }

        #[derive(Debug, Clone, Copy, TryFromPrimitive)]
        #[repr(u8)]
        pub enum $idname {
            $( $variant = $selector, )*
        }
    };
}

macro_rules! request {
    ($reqname: ident $respname: ident $idname: ident { $($selector: literal = $variant: ident($data: ty => $respdata: ty)),* }) => {
        #[derive(Debug, Clone)]
        pub enum $reqname {
            $( $variant($data), )*
        }

        #[derive(Debug, Clone)]
        pub enum $respname {
            $( $variant($respdata), )*
            Unknown { id: u8, data: std::vec::Vec<u8> }
        }

        #[repr(u8)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive)]
        pub enum $idname {
            $( $variant = $selector, )*
        }


        impl $reqname {
            pub fn command(&self) -> $idname {
                match self {
                    $($reqname::$variant(_) => $idname::$variant,)*
                }
            }

            pub fn write(&self, buf: impl std::io::Write) -> std::io::Result<()> {
                Ok(match self {
                    $(Self::$variant(data) => crate::traits::Payload::write(data, buf)?,)*
                })
            }
        }

        impl $respname {
            pub fn command(&self) -> u8 {
                match self {
                    $(Self::$variant(_) => $idname::$variant as _,)*
                    Self::Unknown { id, .. } => *id,
                }
            }

            pub fn read(command: u8, data: impl std::io::Read) -> std::io::Result<Self> {
                let cmd = match $idname::try_from(command) {
                    Ok(cmd) => cmd,
                    _ => {
                        return Ok(Self::Unknown {
                            id: command,
                            data: crate::byte_utils::ReadTail::read_tail(data)?,
                        })
                    }
                };

                Ok(match cmd {
                    $($idname::$variant => Self::$variant(crate::traits::Payload::read(data)?),)*
                })
            }
        }
    };
}

pub(crate) use notification;
pub(crate) use request;
