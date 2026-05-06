use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Role {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NivelMovimento {
    Baixo,
    Medio,
    Alto,
    MuitoAlto,
}

impl NivelMovimento {
    pub fn multiplicador(&self) -> f64 {
        match self {
            NivelMovimento::Baixo => 0.5,
            NivelMovimento::Medio => 1.0,
            NivelMovimento::Alto => 1.3,
            NivelMovimento::MuitoAlto => 1.6,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DiaSemana {
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
    Domingo,
}

impl DiaSemana {
    pub fn nivel(&self) -> NivelMovimento {
        match self {
            DiaSemana::Segunda => NivelMovimento::Baixo,
            DiaSemana::Terca | DiaSemana::Quarta => NivelMovimento::Medio,
            DiaSemana::Quinta | DiaSemana::Sexta => NivelMovimento::Alto,
            DiaSemana::Sabado | DiaSemana::Domingo => NivelMovimento::MuitoAlto,
        }
    }
}