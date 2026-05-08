//! Módulo de modelos de domínio.
//!
//! Contém as structs que representam as entidades do sistema:
//! - [`User`] — Usuários e tipos de permissão
//! - [`Product`] — Produtos/lotes do estoque
//! - [`Role`] — Cargos do sistema
//! - [`Movement`](movement::Movement) — Histórico de movimentações
//! - [`DiaSemana`](consumo::DiaSemana) / [`NivelMovimento`](consumo::NivelMovimento) — Alertas de consumo

pub mod user;
pub mod product;
pub mod role;
pub mod movement;
pub mod consumo;
pub use user::*;
pub use product::*;
pub use role::*;
