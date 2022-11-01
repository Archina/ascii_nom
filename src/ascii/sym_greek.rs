use std::str::FromStr;

use nom::{branch::alt, combinator::map, bytes::complete::tag, IResult};
use strum_macros::EnumString;

use super::sym::{Symbol, SymbolSemantic};

#[allow(non_camel_case_types)]
#[derive(EnumString, Debug)]
pub enum Greek{
    alpha,
    beta,
    gamma,
    Gamma,
    delta,
    Delta,
    epsilon,
    varepsilon,
    zeta,
    eta,
    theta,
    Theta,
    vartheta,
    iota,
    kappa,
    lambda,
    Lambda,
    mu,
    nu,
    xi,
    Xi,
    pi,
    Pi,
    rho,
    sigma,
    Sigma,
    tau,
    upsilon,
    phi,
    Phi,
    varphi,
    chi,
    psi,
    Psi,
    omega,
    Omega
}

pub fn parse_greek(i: &str) -> IResult<&str, Symbol>{
    map(alt((
        alt((
            tag("alpha"),
            tag("beta"),
            tag("gamma"),
            tag("Gamma"),
            tag("delta"),
            tag("Delta"),
            tag("epsilon"),
            tag("varepsilon"),
            tag("zeta"),
            tag("eta"),
            tag("theta"),
            tag("Theta"),
            tag("vartheta"),
        )),
        alt((
            tag("iota"),
            tag("kappa"),
            tag("lambda"),
            tag("Lambda"),
            tag("mu"),
            tag("nu"),
            tag("xi"),
            tag("Xi"),
            tag("pi"),
            tag("Pi"),
            tag("rho"),
            tag("sigma"),
            tag("Sigma"),
            tag("tau"),
            tag("upsilon"),
            tag("phi"),
            tag("Phi"),
            tag("varphi"),
        )),
        tag("chi"),
        tag("psi"),
        tag("Psi"),
        tag("omega"),
        tag("Omega"),
    )), |val: &str| {
        let first_char: char = val.chars().nth(0).unwrap();
        Symbol{
            payload: super::sym::SymbolType::Greek(Greek::from_str(val).unwrap()),
            semantic: if first_char.is_uppercase() {SymbolSemantic::Operator} else { SymbolSemantic::Identifier}
        }
    })(i)
}

impl Greek{
    pub fn as_str(&self) -> &'static str{
        match self{
            Greek::alpha => "&#x3B1;",
            Greek::beta => "&#x3B2;",
            Greek::gamma => "&#x3B3;",
            Greek::Gamma => "&#x393;",
            Greek::delta => "&#x3B4;",
            Greek::Delta => "&#x394;",
            Greek::epsilon => "&#x3B5;",
            Greek::varepsilon => "&#x25B;",
            Greek::zeta => "&#x3B6;",
            Greek::eta => "&#x3B7;",
            Greek::theta => "&#x3B8;",
            Greek::Theta => "&#x398;",
            Greek::vartheta => "&#x3D1;",
            Greek::iota => "&#x3B9;",
            Greek::kappa => "&#x3BA;",
            Greek::lambda => "&#x3BB;",
            Greek::Lambda => "&#x39B;",
            Greek::mu => "&#x3BC;",
            Greek::nu => "&#x3BD;",
            Greek::xi => "&#x3BE;",
            Greek::Xi => "&#x39E;",
            Greek::pi => "&#x3C0;",
            Greek::Pi => "&#x3A0;",
            Greek::rho => "&#x3C1;",
            Greek::sigma => "&#x3C3;",
            Greek::Sigma => "&#x3A3;",
            Greek::tau => "&#x3C4;",
            Greek::upsilon => "&#x3C5;",
            Greek::phi => "&#x3D5;",
            Greek::Phi => "&#x3A6;",
            Greek::varphi => "&#x3C6;",
            Greek::chi => "&#x3C7;",
            Greek::psi => "&#x3C8;",
            Greek::Psi => "&#x3A8;",
            Greek::omega => "&#x3C9;",
            Greek::Omega => "&#x3A9;",
        }
    }
}