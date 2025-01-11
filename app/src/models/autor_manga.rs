use crate::RedSocial;
#[derive(Clone,Default)]
pub struct Autor{
    pub id:ActorId,
    pub nombre:String,
    pub email:String,
    pub foto_perfil:String,
    pub descripcion:String,
    pub idioma_origen:String,
    pub ingresos_totales:f64,
    pub donaciones_recibidas:f64,
    pub fechas_registro:String,
    pub ultima_activad:String,
    pub estado_cuenta:String,
    pub redes_sociales:Vec<RedSocial>
}
