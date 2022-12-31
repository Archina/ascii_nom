extern crate strum;
extern crate strum_macros;

pub mod ascii;
pub mod mathml;


#[cfg(test)]
mod tests {
    use crate::mathml::MathMl;

    #[test]
    fn parse_frac(){
        if let Ok((unparsed, parsed)) = crate::ascii::exp::parse_expression("1/2") {
            // Everything consumed...
            assert_eq!(unparsed.len(), 0);
            assert_eq!("<mfrac><mn>1</mn><mn>2</mn></mfrac>", parsed.to_math_ml());
        } else {
            assert!(false);
        }
    }
    
    #[test]
    fn parse_simple_matrix() {
        if let Ok((unparsed, parsed)) = crate::ascii::exp::parse_expression("[(1,2),(3,4)]") {
            // Everything consumed...
            assert_eq!(unparsed.len(), 0);
            assert_eq!("<mrow><mo>[</mo><mtable columnlines=\"none none\"><mtr><mtd><mn>1</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable><mo>]</mo></mrow>", parsed.to_math_ml());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn parse_simple_addition_matrix() {
        if let Ok((unparsed, parsed)) = crate::ascii::exp::parse_expression("[(1+6,2),(3,4)]") {
            // Everything consumed...
            assert_eq!(unparsed.len(), 0);
            assert_eq!("<mrow><mo>[</mo><mtable columnlines=\"none none\"><mtr><mtd><mn>1</mn><mo>+</mo><mn>6</mn></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable><mo>]</mo></mrow>", parsed.to_math_ml());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn parse_invalid_matrix_as_nested_brackets() {
        if let Ok((unparsed, parsed)) = crate::ascii::exp::parse_expression("[(3,4,6),(3,3)]") {
            // Everything consumed...
            assert_eq!(unparsed.len(), 0);
            assert_eq!("<mrow><mo>[</mo><mrow><mo>(</mo><mn>3</mn><mo>,</mo><mn>4</mn><mo>,</mo><mn>6</mn><mo>)</mo></mrow><mo>,</mo><mrow><mo>(</mo><mn>3</mn><mo>,</mo><mn>3</mn><mo>)</mo></mrow><mo>]</mo></mrow>", parsed.to_math_ml());
        } else {
            assert!(false);
        }
    }
}