use serde::{Serialize, Deserialize};

/// Nível de movimentação esperado para um dia da semana.
///
/// Cada nível possui um multiplicador que ajusta o estoque mínimo
/// recomendado para o dia, permitindo alertas proporcionais à demanda.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NivelMovimento {
    Baixo,
    Medio,
    Alto,
    MuitoAlto,
}

impl NivelMovimento {
    /// Retorna o fator multiplicador do estoque mínimo para este nível.
    ///
    /// - Baixo: 0.5x (metade do min_stock)
    /// - Médio: 1.0x (min_stock padrão)
    /// - Alto: 1.3x (30% acima do min_stock)
    /// - MuitoAlto: 1.6x (60% acima do min_stock)
    pub fn multiplicador(&self) -> f64 {
        match self {
            NivelMovimento::Baixo => 0.5,
            NivelMovimento::Medio => 1.0,
            NivelMovimento::Alto => 1.3,
            NivelMovimento::MuitoAlto => 1.6,
        }
    }

    /// Retorna o nome legível do nível.
    pub fn descricao(&self) -> &str {
        match self {
            NivelMovimento::Baixo => "Baixo",
            NivelMovimento::Medio => "Médio",
            NivelMovimento::Alto => "Alto",
            NivelMovimento::MuitoAlto => "Muito Alto",
        }
    }
}

/// Dias da semana com mapeamento para nível de movimentação.
///
/// Utilizado pelo sistema de alerta para calcular o estoque mínimo
/// ajustado conforme o dia da semana atual.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    /// Retorna o nível de movimentação esperado para este dia.
    ///
    /// - Segunda: Baixo
    /// - Terça/Quarta: Médio
    /// - Quinta/Sexta: Alto
    /// - Sábado/Domingo: Muito Alto
    pub fn nivel(&self) -> NivelMovimento {
        match self {
            DiaSemana::Segunda => NivelMovimento::Baixo,
            DiaSemana::Terca | DiaSemana::Quarta => NivelMovimento::Medio,
            DiaSemana::Quinta | DiaSemana::Sexta => NivelMovimento::Alto,
            DiaSemana::Sabado | DiaSemana::Domingo => NivelMovimento::MuitoAlto,
        }
    }

    /// Converte o dia da semana do chrono (Weekday) para o enum DiaSemana.
    pub fn from_chrono(weekday: chrono::Weekday) -> Self {
        match weekday {
            chrono::Weekday::Mon => DiaSemana::Segunda,
            chrono::Weekday::Tue => DiaSemana::Terca,
            chrono::Weekday::Wed => DiaSemana::Quarta,
            chrono::Weekday::Thu => DiaSemana::Quinta,
            chrono::Weekday::Fri => DiaSemana::Sexta,
            chrono::Weekday::Sat => DiaSemana::Sabado,
            chrono::Weekday::Sun => DiaSemana::Domingo,
        }
    }
}
