// a egg example staturation and e-matching

// ; target language 
// ; number:=  <DIGIT>+
// ; var := <ALPH>
// ; op    := {*, /, <<, +, -}
// ; calc-expression-1 := var | number
// ; calc-expression-3 :=  \(op calc-expression  calc-expression\)

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use egg::{*, rewrite as rw};


fn main()
{
    let prog_list_fpath = std::env::args().nth(1).expect("no input program list file");
    let rules: &[Rewrite<SymbolLang, ()>] = &[
        // rw!("commute-add"; "(+ ?x ?y)" => "(+ ?y ?x)"),
        // rw!("commute-mul"; "(* ?x ?y)" => "(* ?y ?x)"),
        // rw!("shift-1"; "(* ?x 2)" => "(>> ?x 1)"),
    
        // rw!("add-0"; "(+ ?x 0)" => "?x"),
        // rw!("mul-0"; "(* ?x 0)" => "0"),
        // rw!("mul-1"; "(* ?x 1)" => "?x"),
        rw!("test1"; "(/ (* ?x ?y) ?z)" => "(* ?x (/ ?y ?z))"),
        rw!("test2"; "(/ ?x ?x)" => "1"),
        rw!("test3"; "?x" => "(* ?x 1)")
    ];
    
    // let start = "(/ (* (/ 2 3) (/ 3 2)) 1)".parse().unwrap();
    
    let p = Path::new(&prog_list_fpath);
    let prog_list = match File::open(&p) {
        Err(why) => panic!("couldn't open {}: {}", prog_list_fpath, why),
        Ok(file) => io::BufReader::new(file).lines(),
    };
    // let mut egraph: EGraph<_, _> = Default::default();
    for line in prog_list {
        if let Ok(raw_prog) = line {
            // egraph.strategy = Strategy::GenericJoin;
            let p = raw_prog.parse().unwrap();
            let runner = Runner::default().with_expr(&p).run(rules);
            let extractor = Extractor::new(&runner.egraph, AstSize);
            println!("{}", runner.egraph.dot());
            runner.egraph.dot().to_svg("./out.svg").unwrap();
            let (best_cost, best_expr) = extractor.find_best(runner.roots[0]);
            print!("Original program: {} \n ====> Optimized {}, with cost {} \n", raw_prog, best_expr, best_cost);
        }
    }

}
