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
    fn parse_vector(){
        if let Ok((unparsed, parsed)) = crate::ascii::exp::parse_expression("((a),(s),(d))") {
            // Everything consumed...
            assert_eq!(unparsed.len(), 0);
            assert_eq!("<mrow><mo>(</mo><mtable columnlines=\"none\"><mtr><mtd><mi>a</mi></mtd></mtr><mtr><mtd><mi>s</mi></mtd></mtr><mtr><mtd><mi>d</mi></mtd></mtr></mtable><mo>)</mo></mrow>", parsed.to_math_ml());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn parse_transposed_vector(){
        if let Ok((unparsed, parsed)) = crate::ascii::exp::parse_expression("((2,4,5))") {
            // Everything consumed...
            assert_eq!(unparsed.len(), 0);
            assert_eq!("<mrow><mo>(</mo><mtable columnlines=\"none none none\"><mtr><mtd><mn>2</mn></mtd><mtd><mn>4</mn></mtd><mtd><mn>5</mn></mtd></mtr></mtable><mo>)</mo></mrow>", parsed.to_math_ml());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn parse_nested_matrix(){
        if let Ok((unparsed, parsed)) = crate::ascii::exp::parse_expression("((((a),(b)),2),(3,4))") {
            // Everything consumed...
            assert_eq!(unparsed.len(), 0);
            assert_eq!("<mrow><mo>(</mo><mtable columnlines=\"none none\"><mtr><mtd><mrow><mo>(</mo><mtable columnlines=\"none\"><mtr><mtd><mi>a</mi></mtd></mtr><mtr><mtd><mi>b</mi></mtd></mtr></mtable><mo>)</mo></mrow></mtd><mtd><mn>2</mn></mtd></mtr><mtr><mtd><mn>3</mn></mtd><mtd><mn>4</mn></mtd></mtr></mtable><mo>)</mo></mrow>", parsed.to_math_ml());
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

    #[test]
    fn parse_matrix_as_layout(){
        if let Ok((unparsed, parsed)) = crate::ascii::exp::parse_expression("{(2x,+,17y,=,23),(x,-,y,=,5):}") {
            assert_eq!(unparsed.len(), 0);
            assert_eq!("<mrow><mo>{</mo><mtable columnlines=\"none none none none none\"><mtr><mtd><mn>2</mn><mi>x</mi></mtd><mtd><mo>+</mo></mtd><mtd><mn>17</mn><mi>y</mi></mtd><mtd><mo>=</mo></mtd><mtd><mn>23</mn></mtd></mtr><mtr><mtd><mi>x</mi></mtd><mtd><mo>-</mo></mtd><mtd><mi>y</mi></mtd><mtd><mo>=</mo></mtd><mtd><mn>5</mn></mtd></mtr></mtable></mrow>", parsed.to_math_ml());
        } else {
            assert!(false);
        }
    }

    fn parse_sum(){

    }

    fn parse_sum_with_index(){

    }

    #[test]
    fn parse_sum_with_limit(){
        if let Ok((unparsed, parsed)) = crate::ascii::exp::parse_expression("sum^k") {
            // Everything consumed...
            println!("{}", parsed.to_math_ml());
            assert_eq!(unparsed.len(), 0);
            // assert_eq!("<mrow><mo>[</mo><mrow><mo>(</mo><mn>3</mn><mo>,</mo><mn>4</mn><mo>,</mo><mn>6</mn><mo>)</mo></mrow><mo>,</mo><mrow><mo>(</mo><mn>3</mn><mo>,</mo><mn>3</mn><mo>)</mo></mrow><mo>]</mo></mrow>", parsed.to_math_ml());
        } else {
            assert!(false);
        }
    }
}