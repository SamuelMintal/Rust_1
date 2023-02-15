use std::ops;

#[derive(Debug, Clone)]
pub struct Element {
    pub coef: i32,
    pub var: String,
    pub pwr: i32,
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        if self.coef == other.coef && self.var == other.var && self.pwr == other.pwr {
            return true;
        }
        return false;
    }
} 

#[derive(Debug, Clone)]
pub struct Polynomial {
    poly: Vec<Element>,    
}

impl Polynomial {    

    /*pub fn new() -> Polynomial {
        return Polynomial { poly: Vec::new() };
    }*/

    pub fn builder() -> PolynomialBuilder {        
        return PolynomialBuilder { poly: Vec::new() };
    }
}

impl ops::Add<Polynomial> for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Polynomial) -> Polynomial {
        
        let mut pb: PolynomialBuilder = PolynomialBuilder::from_polynomial(self);
        
        for element in rhs.poly {
            pb = pb.add_from_elem(element);
        }

        return pb.build();
    }
}

impl PartialEq for Polynomial {
    fn eq(&self, other: &Self) -> bool {
        
        if self.poly.len() != other.poly.len() {
            return false;
        }

        for self_elem in &self.poly {
            let mut found = false;

            for other_elem in &other.poly {
                if self_elem == other_elem {
                    found = true;
                    break;
                }
            }

            if !found {
                return false;
            }
        }

        return true;
    }
}

pub struct PolynomialBuilder {
    poly: Vec<Element>,    
}

impl PolynomialBuilder {

    fn from_polynomial(polynomial: Polynomial) -> PolynomialBuilder {
        return PolynomialBuilder { poly: polynomial.poly };
    }

    fn add_from_elem(mut self, element: Element) -> PolynomialBuilder { 
        
        return self.add(element.coef, element.var.as_str(), element.pwr);
    }

    fn is_in_poly(self: &Self, var: &str, pwr: i32) -> Option<usize> {

        for i in 0..self.poly.len() {
            if self.poly[i].var == var && self.poly[i].pwr == pwr {
                return Some(i);
            }
        };

        return None;
    }

    pub fn add(mut self, coef: i32, var: &str, pwr: i32) -> PolynomialBuilder {

        match self.is_in_poly(var, pwr) {
            Some(i) => {                
                self.poly[i].coef += coef;
            }
            None => {                
                self.poly.push( Element { coef: coef, var: var.to_string(), pwr: pwr } );
            }
        };        

        return self;
    }

    pub fn build(self) -> Polynomial {

        let mut cleared_poly: Vec<Element> = Vec::new();

        for poly_elem in self.poly {
            if poly_elem.coef != 0 {
                cleared_poly.push(poly_elem);
            }
        }

        return Polynomial { poly: cleared_poly };
    }
}