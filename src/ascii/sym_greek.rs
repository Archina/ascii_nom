use std::str::FromStr;

use nom::{branch::alt, combinator::map, bytes::complete::tag};
use strum_macros::EnumString;

use super::sym::Symbol;

#[derive(EnumString, Debug)]
pub enum Greek{
    Alpha,
    Beta,
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

pub fn parse_greek(i: &str) -> nom::IResult<&str, Symbol>{
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
    )), |val| {
        Symbol::Greek(Greek::from_str(val).unwrap())
    })(i)
}
