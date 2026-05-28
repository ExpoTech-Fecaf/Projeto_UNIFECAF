use gerenciamento_de_estoque::models::consumo::{DiaSemana, NivelMovimento};
use chrono::Weekday;

#[test]
fn test_nivel_multiplicador_e_descricao() {
    assert_eq!(NivelMovimento::Baixo.multiplicador(), 0.5);
    assert_eq!(NivelMovimento::Medio.multiplicador(), 1.0);
    assert_eq!(NivelMovimento::Alto.multiplicador(), 1.3);
    assert_eq!(NivelMovimento::MuitoAlto.multiplicador(), 1.6);

    assert_eq!(NivelMovimento::Baixo.descricao(), "Baixo");
    assert_eq!(NivelMovimento::Medio.descricao(), "Médio");
    assert_eq!(NivelMovimento::Alto.descricao(), "Alto");
    assert_eq!(NivelMovimento::MuitoAlto.descricao(), "Muito Alto");
}

#[test]
fn test_dia_semana_from_chrono_and_nivel() {
    assert_eq!(DiaSemana::from_chrono(Weekday::Mon).nivel(), NivelMovimento::Baixo);
    assert_eq!(DiaSemana::from_chrono(Weekday::Tue).nivel(), NivelMovimento::Medio);
    assert_eq!(DiaSemana::from_chrono(Weekday::Wed).nivel(), NivelMovimento::Medio);
    assert_eq!(DiaSemana::from_chrono(Weekday::Thu).nivel(), NivelMovimento::Alto);
    assert_eq!(DiaSemana::from_chrono(Weekday::Fri).nivel(), NivelMovimento::Alto);
    assert_eq!(DiaSemana::from_chrono(Weekday::Sat).nivel(), NivelMovimento::MuitoAlto);
    assert_eq!(DiaSemana::from_chrono(Weekday::Sun).nivel(), NivelMovimento::MuitoAlto);
}
