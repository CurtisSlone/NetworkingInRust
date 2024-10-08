#[derive(Debug, PartialEq)]
struct Kubeconfig {
    port: u8,
    healthz_port: u8,
    max_pods: u8
}

pub trait Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer;
}

impl Serialize for Kubeconfig {
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
    {
        let mut state = serializer.serialize_struct("KubeConfig",3)?;
        state.serialize_field("port", &self.port)?;
        state.serialize_field("healthz_port", &self.healthz_port)?;
        state.serialize_field("max_pods", &self.max_pods)?;
        state.end()
    }
}

pub trait Deserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Desrializer<'de>;
}

pub trait Visitor<'de>: Sized {
    type Value;
    fn expecting(&self, formatter: &mut Formatter) -> Result;
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value,
    E> where E: Error;
    {}
}

impl<'de> Deserialize<'de> for KubeConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserialize<'dE>
        {
            enum field { Port, HealthzPort, MaxPods};

            impl<'de>Deserialize<'de> for Field {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where D:Desrializer<'de>
                    {
                        struct FieldVisitor;

                        impl<'de> Visitor<'de> for FieldVisitor {
                            type Value = field;
                            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                                formatter.write_str("`port`, `healthz_port`, or `max_pods`")
                            }

                            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                                where E: Error
                            {
                                match value {
                                    "port" => Ok(Field::Port),
                                    "healthz_port" => Ok(Field::HealthzPort),
                                    "max_pods" => Ok(Field::MaxPods),
                                    _ => Err(E::unknown_field(value, FIELDS))
                                }
                            }
                        }

                        deserializer.deserialize_identifier(FieldVisitor)
                    }
            }
        }
}

fn main() {

}
